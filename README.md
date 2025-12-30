[![crates.io](https://img.shields.io/crates/v/problemo?color=%23227700)](https://crates.io/crates/problemo)
[![docs.rs](https://img.shields.io/badge/docs.rs-latest?color=grey)](https://docs.rs/problemo)

Problemo
========

This library aims to improve the experience of working with Rust's std `Error` trait by allowing for deep causation chains and arbitrary attachments with the goal of making it easy and rewarding to return richly-typed errors for callers to inspect and handle.

Specifically, we've identified three features missing from std:

## 1. Causation Chains

Firstly, we want to be able to set any error as the cause of any other error. This allows us to build causation chains that we can inspect and handle as needed.

The std `Error` trait does have a `source()` function, but the trait does not provide a way for setting the source. Indeed, it is an optional feature.

A common way to express causation is to create an `enum` error type where each variant contains the source error. Libraries such as [derive_more](https://github.com/JelteF/derive_more) and [thiserror](https://github.com/dtolnay/thiserror) make it easy enough to do so. Unfortunately this solution requires us to define variants for *all* the error types that *can* be our source. This bookkeeping become tedious as new error types must be added when functionality grows while old, no-longer-used error types linger on and clutter the codebase. And since the same lower-level errors crop up again and again these enums have a lot of duplication. The biggest problem is that further nesting is possible only if your sources are themselves enums, so the causation chain cannot be arbitrary.

Problemo's solution is to introduce a wrapper type for errors, `Problem`, which contains a causation chain of any error type (by relying on the familiar [`Box<dyn Error>` mechanism](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)). Simply call `via()` to add an error to the front of the chain. This mechanism does not replace the existing `source()` mechanism but instead complements it as we provide APIs that make it easy to iterate our chaining as well as recursively traversing `source()`.

Because it's so easy to chain errors with `via()` an elegant pattern emerges. Instead of managing one big `enum`, we can create simple, reusable "tag" error types (just an empty `struct`), which can be chained in front of any error. This is so common that we provide the `tag_error!` macro to make it easy to create them. Example:

```rust
use problemo::*;

// The second optional argument is the Display message
// (defaults to the type name)
tag_error!(OperatingSystemError, "operating system");

fn read_the_file(path: &str) -> Result<String, Problem> {
    std::fs::read_to_string(path).via(OperatingSystemError)
}

fn main() {
    if let Err(problem) = read_the_file("my-file.txt") {
        if problem.has_type::<OperatingSystemError>() {
            println!("Your computer is broken!");
        } else {
            println!("Could not read. Try again?");
        }
    }
}
```

If you prefer to group error types together then an `enum` of tags also works (using `has()` instead of `has_type()`). You can use [derive_more](https://github.com/JelteF/derive_more) or [thiserror](https://github.com/dtolnay/thiserror) to define it easily. Example:

```rust
use {derive_more::*, problemo::*};

// Note that we need to implement PartialEq for has() to work
#[derive(Debug, Display, Error, PartialEq, Eq)]
enum OperatingSystemError {
    #[display("low-level I/O")]
    IO,

    #[display("network")]
    Network,
}

// This is similar to just using:
//   tag_error!(IoError);
//   tag_error!(NetworkError);
// 
// The advantage of grouping them together in an enum is that we can do:
//   has_type::<OperatingSystemError>()

fn read_the_file(path: &str) -> Result<String, Problem> {
    std::fs::read_to_string(path).via(OperatingSystemError::IO)
}

fn main() {
    if let Err(problem) = read_the_file("my-file.txt") {
        if problem.has(OperatingSystemError::IO) {
            println!("Your computer is broken!");
        }
        ...
    }
}
```

> Note that `Problem` does not itself implement the std `Error` trait, though it does implement `Debug` and `Display`. This is due to a current limitation in Rust's type system that [may have a workaround in the future](https://std-dev-guide.rust-lang.org/policy/specialization.html). Until then simply call `into_error()` when you need a `Problem` to implement `Error`.

## 2. Attachments

Secondly, we want to be able to attach additional, typed values to *any* error. This allows us to provide additional context for error handling, such as backtraces, locations and spans in source files, and request IDs, as well as custom representations, formatting for different environments, etc.

The std `Error` trait supports exactly three attachments: the optional `source()` mentioned above as well as the required `Debug` and `Display` string representations.

An extensible solution would require storing and exposing APIs for additional attachments in your error types, possibly via a trait and/or wrapper types.

Our solution is remarkably trivial. Because we already have a wrapper type, `Problem`, we've simply included attachments in it as a vector of `Box<dyn Any>`. To add an attachment you just need to call `with(my_attachment)`. We also provide APIs that make it easy to look for specific attachment types. Example:

```rust
use problemo::*;

tag_error!(OperatingSystemError, "operating system");
tag_error!(ParseError, "parse");

struct Location {
    row: usize,
    column: usize,
}

fn parse_file(path: &str) -> Result<(), Problem> {
    let contents = std::fs::read_to_string(path).via(OperatingSystemError)?;
    ...
    Err(ParseError.into_problem().with(Location { row, column }))
}

fn main() {
    if let Err(problem) = parse_file("my-file.txt") {
        println!("{}", problem);
        if let Some(location) = problem.attachment_of::<Location>() {
            println!("  at {}/{}", location.row, location.column);
        }
    }
}
```

Because attaching backtraces is very common we provide a `with_backtrace()` convenience method.

## 3. Error Accumulation

There are cases in which functions should be able to return more than one error. A classic example is parsing, as there might be multiple syntax and grammar errors in the input and callers (and users) would be interested in all of them. Another example is a function that distributes work among threads, where each thread could encounter different errors and all of them must be handled by the caller.

A common solution is to create a custom error type that internally stores multiple errors. Or, more simply, the error could just be a `Vec` of errors.

But our requirement goes beyond mere multiplicity. In some cases callers might care only that the function succeeds, in which case it would be more efficient to fail on the first error, a.k.a. "fail fast". We might also sometimes prefer to "stream" the errors rather than accumulating them in memory. For example, our parser might emit thousands of errors on a bad input. It would be more memory-efficient to print them out as they arrive.

Problemo's solution is the `ProblemReceiver` trait. If we want to accumulate the errors we can use the `Problems` type, which implements it by "swallowing" the errors into a vector. If we want to fail on the first error we can use the `FailFast` type, which implements it by simply returning the error. `Problems` also supports a list of optional "critical" error types: If it encounters one of these it fails fast instead of swallowing it.

Using this trait does involve some awkwardness. We believe it's worth it for the flexibility and opportunities for optimization.

The first challenge is that this is an [inversion-of-control](https://en.wikipedia.org/wiki/Inversion_of_control) design, meaning that *we* have to provide the `ProblemReceiver` implementation. Most commonly this would be as a function argument.

Secondly, an error-accumulating function's `Result` might actually be `Ok` even if there are errors because they had all been swallowed by the receiver. The first consequence is that such a function needs to be able to return *something* with `Ok`. This could be a partial result, which can be useful in itself (e.g. we can show the user what we succeeded in parsing in spite of the errors). If that's impossible or irrelevant then we can just return an `Option::None`. The second consequence is that upon getting an `Ok` the caller will *still* need to check for accumulated errors. `Problems` has a `check()` function that does just that.

A useful advantage of inversion-of-control is that the caller owns the receiver. This means that it can be reused: You can call multiple functions with one `Problems` and then handle all the accumulated errors at once.

Proper use of this feature does require some discipline. To make it all a bit easier we provide a few friendly APIs. Example:

```rust
use problemo::*;

/// By our convention we'll put the receiver as the *last* argument
fn read_files<ProblemReceiverT>(
    paths: &[&str],
    problems: &mut ProblemReceiverT,
) -> Result<Vec<String>, Problem>
where
    ProblemReceiverT: ProblemReceiver,
{
    let mut strings = Vec::default();
    for path in paths {
        // give_ok() is like ok() but will give the problem to the receiver
        if let Some(string) = std::fs::read_to_string(path)
            .via(LowLevelError)
            .give_ok(problems)?
        {
            strings.push(string);
        }
    }

    // If we had errors then this would be a partial result;
    // i.e. not all files were read
    Ok(strings)
}

fn main() -> Result<(), Problem> {
    let mut problems = Problems::default();
    let strings = read_files(&["file1.txt", "file2.txt"], &mut problems)?;
    
    // When using Problems the call above will *never* return Err
    // (it swallowed all the errors)
    // We thus *must* call check() here
    problems.check()?;

    // By contrast, we can trust that FailFast will always return Err on error
    let strings = read_files(&["file3.txt", "file4.txt"], &mut FailFast)
}
```

Some Other Features
-------------------

### Working with std `Result`

`?` on a `Result` will just work if the error is a std `Error` type (because `Problem` implements `From<Error>`). In effect this starts a causation chain.

We furthermore provide an easy-to-use extension trait for std `Result` so you can do `via()` and `with()` on it directly without having to `map_err()`. We've used it in some of the examples above.

### Just Strings

Do you only care about error messages and not error types? Then you can do:

```rust
return Err("I failed".into_problem());
```

Fancier version:

```rust
return Err(format!("{} failed", subject).into_problem());
```

Behind the scenes your string will be wrapped in `MessageError`, which is just a string newtype.

Or be a bit less lazy and use our `message_error!()` macro to create more specific message error types. That way callers would be able to differentiate between them. It would take just a bit more work from you. You can do it! We believe in you! Example:

```rust
use problemo::*;

// The second optional argument is a prefix for Display
message_error!(InvalidPathError, "invalid path");

fn read_the_file(path: &str) -> Result<String, Problem> {
    if path.is_empty() {
        return Err(InvalidPathError::new(path).into());
    }
    ...
}
```

### Unmovable Errors

Is your error unmovable? If so, it cannot be captured in a `Problem`. This is famously the case with `PoisonError`, which you can get from locking a `Mutex`. We can, however, capture such an error's `Display` representation, which we enable with `into_message_problem()` (which uses the `MessageError` type mentioned above) as well as `into_concurrency_problem()` (which uses `ConcurrencyError`).

Alternatively you can use `map_into_problem()` and provide your own conversion function. As it simplest it can conserve the `Display`, but it can also have fields in which you conserve additional information. The point is to translate the error into a movable one.

FAQ
---

### Why doesn't `Problem` wrap a concrete error type? Wouldn't it better to allow the compiler to check for correct usage?

Our experience in large projects with error-handling libraries that require typed errors has led us to believe that it's a cumbersome and ultimately pointless requirement, especially for high-level functions that might have long causation chains. That's where the meat is. In practice just having the compiler test that the topmost "tag" is correct guarantees very little if you're not actually handling that error. It's just clutter.

### Why doesn't Problemo come with macros such as `bail!()`?

Most error-handling libraries have macros that optimize the creation and returning of errors so that it would take the smallest number of keyboard strokes. Problemo instead prefers verbosity, clarity, and debuggability by suggesting explicit function calls.

We use macros only when functions *can't* work. Generally speaking our code is deliberately non-magical and should be easy for any Rust programmer to understand. Error handling is foundational and should not be a mysterious black box.

You are of course free to create your own macros for Problemo but we don't want to encourage their use in the published API.

### I like Problemo but it's missing my favorite feature!

That's not a question! Anyway, we are happy to hear your suggestions. Please be nice about it and do keep in mind that we want to keep Problemo lean, simple, and focused on the essentials. If the feature is something that can be built on top of Problemo, perhaps as a supplementary library, then that will likely be the preferred route.

### Spanish?

Actually it's [Spanglish](https://en.wikipedia.org/wiki/No_problem#No_problemo). And even more actually it's [Esperanto](https://glosbe.com/eo/en/problemo). Mi havas naŭdek naŭ problemojn, sed hundino ne estas unu.

### "AI"?

Please, no.

Popular Alternatives
--------------------

* [error-stack](https://github.com/hashintel/hash/tree/main/libs/error-stack), like Problemo, supports chaining and attachments. It does, however, require you to provide a concrete error type for your returns, which it calls the "context". It supports returning groups of errors as long as they are of that same "context" type. It also features backtraces and pretty printing.

* [rootcause](https://github.com/rootcause-rs/rootcause) also supports chaining and attachments. It supports both type-less wrappers like Problemo as well as error-stack-style typed errors. It comes with a customizable error formatter and many other features. Its scope is broad and its relatively complex.

* [anyhow](https://github.com/dtolnay/anyhow) is a one-trick pony. It's essentially a spruced-up `Box<dyn Error>` that is optimized for reduced memory usage. It does add support for two attachment types: a backtrace, which is handled implicitly and automatically, and what is calls a "context", which is an implementation of std `Display`. Anyhow does not support chaining or other attachment types.

* [SNAFU](https://github.com/shepmaster/snafu) works similarly to Anyhow in practice but takes a different design approach by introducing its own set of traits as a replacement for std `Error` while also allowing for compatibility with it. This allows you to build custom, rich error types on top of SNAFU. Otherwise it also supports backtraces and a "context" attachment.

* [eyre](https://github.com/eyre-rs/eyre) is similar to (and compatible with) Anyhow but also supports chaining and pretty printing.

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](https://github.com/tliron/problemo/blob/main/LICENSE-APACHE)
* [MIT license](https://github.com/tliron/problemo/blob/main/LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
