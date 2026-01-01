[![crates.io](https://img.shields.io/crates/v/problemo?color=%23227700)](https://crates.io/crates/problemo)
[![docs.rs](https://img.shields.io/badge/docs.rs-latest?color=grey)](https://docs.rs/problemo)

Problemo
========

This library aims to improve the experience of working with Rust's std `Error` trait by allowing for deep causation chains, arbitrary attachments, and error accumulation with the goal of making it easy and rewarding to return richly-typed errors for callers to inspect and handle.

Specifically, we've identified three features missing from std:

## 1. Causation Chains

Firstly, we want to be able to set any error as the cause of any other error. This allows us to build causation chains that we can inspect and handle as needed.

The std `Error` trait does have a `source()` function, but the trait does not provide a way for setting the source. Indeed, it is an optional feature.

A common way to express causation is to create an `enum` error type where each variant contains the source error. Libraries such as [derive_more](https://github.com/JelteF/derive_more) and [thiserror](https://github.com/dtolnay/thiserror) make it easy enough to do so. Unfortunately this solution requires us to define variants for *all* the error types that *can* be our source. This bookkeeping become tedious as new error types must be added when functionality grows while old, no-longer-used error types linger on and clutter the codebase. And since the same lower-level errors crop up again and again these enums have a lot of duplication. The biggest problem is that further nesting is possible only if your sources are themselves enums, so the causation chain cannot be arbitrary.

Problemo's solution is to introduce a wrapper type for errors, `Problem`, which is simply a causation chain of any error type, relying on the familiar [`Box<dyn Error>`](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)  mechanism. Call `via()` on it to add an error to the front of the chain. This mechanism does not replace the std `source()` but instead complements it as we provide APIs that make it easy to iterate our chaining as well as recursively traversing `source()`.

> Note that `Problem` does not itself implement the std `Error` trait, though it does implement `Debug` and `Display`. This is due to a current limitation in Rust's type system that [may have a workaround in the future](https://std-dev-guide.rust-lang.org/policy/specialization.html). Until then simply call `into_error()` when you need a `Problem` to implement `Error`.

### Tag Errors

Because it's so easy to chain errors with `via()` an elegant pattern emerges. Instead of managing one big `enum` we can create simple, reusable "tag" error types (they are just an empty `struct`), which can be chained in front of any error. This is so common that we provide the `tag_error!` macro to make it easy to create them. Example:

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

Problemo comes with a bunch of commonly used tag error types in the `common` module.

### Chain Traversal

If we care about the causes of the error then we can traverse the causation chain with `cause_of_type()`, `under()`, and `iter_under()`:

```rust
if let Some(cause) = problem.cause_of_type::<OperatingSystemError>() {
    println!("Your computer is broken!");
    for cause in cause.iter_under() {
        println!("  because: {}", cause.error);
    }
}
```

### Grouping Error Types

Tag errors are unrelated to each other in the type system. Sometimes, however, it can be useful to group them together as variants of a basic type. In other words, an `enum`.

We can use [derive_more](https://github.com/JelteF/derive_more) to easily define such an `enum` and then use Problemo's `has()` and `cause_for()` instead of `has_type()` and `cause_of_type()`. Example:

```rust
use {derive_more::*, problemo::*};

// Note that we need to implement PartialEq for has() to work
#[derive(Debug, Display, Error, PartialEq)]
enum OperatingSystemError {
    #[display("I/O")]
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
        if problem.has(&OperatingSystemError::IO) {
            println!("Your computer is broken!");
        }
        ...
    }
}
```

### Message Errors

Introducing an `enum` might be overkill if the differences between our variations are merely cosmetic.

For this simpler use case we provide the `message_error!()` and `static_message_error!()` macros, which are similar to `tag_error!()` but with the option of allowing us to change their `Display`. They are simply string newtypes. Example:

```rust
use problemo::*;

// The second optional argument is a prefix for Display
static_message_error!(InvalidPathError, "invalid path");

fn read_the_file(path: &str) -> Result<String, Problem> {
    if path.is_empty() {
        return Err(InvalidPathError::new("empty").into());
    } else if !path.starts_with('/') {
        return Err(InvalidPathError::new("not absolute").into());
    }
    ...
}
```

Problemo comes with a bunch of commonly used message error types in the `common` module. One of them is simply called `MessageError`. We provide a convenience function:

```rust
return Err("I failed".into_message_problem());
```

Fancier version:

```rust
return Err(format!("{} failed", subject).into_message_problem());
```

That's a bit lazy but it will work in a pinch. We still recommend using one of the `common` message error types or defining your own so that callers can better differentiate them:

```rust
return Err(common::InvalidError::new("wrong type").into());
```

## 2. Attachments

Secondly, we want to be able to attach additional, typed values to *any* error. This allows us to provide contextual information for handling the error, such as backtraces, locations and spans in source files, request IDs, as well as custom representations, formatting rules for different environments, etc.

The std `Error` trait supports exactly three attachments: the optional `source()` mentioned above as well as the required `Debug` and `Display` textual representations.

An extensible solution would require storing and exposing APIs for additional attachments in our error types via a traits and/or wrapper types.

Problemo's solution is remarkably trivial. Because we already have a wrapper type, `Problem`, we've simply included attachments as a vector of `Box<dyn Any>` for every cause in the chain. To add an attachment you just need to call `with()`. We also provide APIs that make it easy to find specific attachment types. Example:

```rust
use problemo::*;

tag_error!(OperatingSystemError, "operating system");
tag_error!(ParseError, "parse");

struct Location {
    row: usize,
    column: usize,
}

fn parse_file(path: &str) -> Result<(), Problem> {
    let content = std::fs::read_to_string(path).via(OperatingSystemError)?;
    ...
    // Note the into_problem() function, which is equivalent to Problem::from(error)
    return Err(ParseError.into_problem().with(Location { row, column }));
    ...
}

fn main() {
    if let Err(problem) = parse_file("my-file.txt") {
        println!("{}", problem);
    
        // Note that this will return the first Location in the causation chain;
        // Use attachments_of_type() if you want all of them 
        if let Some(location) = problem.attachment_of_type::<Location>() {
            println!("  at {}/{}", location.row, location.column);
        }
    }
}
```

### Data vs. Metadata

Attachments are an error-handling super power as they allow us to separate the error "data" from its contextual "metadata", which simplifies and declutters error handling.

Consider that in our parser example above we might have various error types happening at a "location". Without the attachments feature we would probably have create something like a `Locatable` trait and implement it for all potential error types. And if it's an external error type we would have to also wrap it (because of the [orphan rules](https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules)). It's all quite painful boilerplate. We know because we've done it a lot. This pain is the raison d'être for Problemo.

### It's Probably Metadata

A best practice emerges: When defining an error `struct`, for every one of its fields we ask ourselves if it's contextual metadata. In less philosophical terms: Could this field potentially be relevant to other error types? If so, we make it an attachment type instead of a field. It then becomes a reusable building block for providing context to *any* error type.

If we follow this rule of thumb we find out that most if not all of our error fields are better defined as attachments. This is why tag errors are so useful: When all the data lives in attachments then we can get away with an empty `struct` for the actual error.

Indeed, this is so common that we provide the `attachment!()`, `string_attachment!()`, and `static_string_attachment!()` macros to make it easy to define newtypes for single-value attachments. Example:

```rust
use problemo::*;

tag_error!(UrlError, "URL");
string_attachment!(UrlAttachment);

fn read_url(url: &str) -> Result<String, Problem> {
    let content = reqwest::blocking::get(url).via(UrlError).with(UrlAttachment::new(url))?;
    ...
}
```

Finally, because attaching backtraces is very common we provide a `with_backtrace()` convenience method (enabled by the `backtrace` feature).

## 3. Error Accumulation

There are cases in which functions should be able to return more than one error. A classic example is parsing, in which there might be multiple syntax and grammar errors in the input. Callers (and users) would be interested in all of them. Another example is a function that distributes work among threads, in which each thread could encounter different errors. Again, all of them must be made known to the caller.

A common solution is to create a custom error type that internally stores multiple errors. Or even more simply the error could just be a vector of errors.

But our requirement goes beyond mere multiplicity. In some cases callers might care only that the function succeeds, in which case it would be more efficient to fail on the first error, a.k.a. "fail fast". We might also sometimes prefer to "stream" the errors instead of storing them all in memory. For example, our parser might emit thousands of errors on a bad input so it would be more memory-efficient to print them out as they arrive. In other words, we should be able to accumulate them into the terminal instead of into memory.

Problemo's solution is the `ProblemReceiver` trait. If we want to store the errors we can use the `Problems` type, which implements the trait by "swallowing" the errors into a vector. If we want to fail on the first error we can use the `FailFast` type, which implements the trait by simply returning the error. `Problems` also supports a list of optional "critical" error types: If it encounters one of these it fails fast instead of swallowing.

### Challenges

Using this trait does involve some awkwardness and discipline. We believe it's worth it for the flexibility and for providing opportunities for optimization.

The first challenge is that this is an [inversion-of-control](https://en.wikipedia.org/wiki/Inversion_of_control) design, meaning that *we* have to provide the `ProblemReceiver` implementation. Most commonly this would be as a function argument.

The second challenge an error-accumulating function's `Result` might actually be `Ok` even if there are errors because they had all been swallowed by the receiver. The first consequence is that such a function needs to be able to return *something* with `Ok`. This could be a partial result, which can be useful in itself (e.g. we can show the user what we succeeded in parsing in spite of the errors). It could be an empty collection or. If that's impossible or irrelevant then it could be a `None`, which case we have to make sure to return an `Option`. The second consequence is that upon getting an `Ok` the caller will *still* need to check for accumulated errors. `Problems` has a `check()` function that does just that.

A useful advantage of inversion-of-control is that the caller owns the receiver. This means that it can be reused: A function can pass the same receiver to other functions that it calls. The caller, too, can call multiple functions with one `Problems` and then handle all the accumulated errors at once. 

To make all of this a bit easier we provide a few friendly APIs. Example:

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
        // give_ok() is like ok() but will give the problem to the receiver;
        // Note that we still want to use "?" in order to support a fast fail
        if let Some(string) = std::fs::read_to_string(path)
            .via(LowLevelError)
            .give_ok(problems)?
        {
            strings.push(string);
        }
    }

    // If we had errors then this would be a partial result
    // (i.e. not all files were read)
    Ok(strings)
}

fn main() -> Result<(), Problem> {
    let mut problems = Problems::default();
    let strings = read_files(&["file1.txt", "file2.txt"], &mut problems)?;
    
    // When using Problems the call above will *never* return Err
    // (it swallowed all the errors)
    // Thus we thus *must* call check() here
    problems.check()?;

    // By contrast, we can trust that FailFast will always return Err on error
    let strings = read_files(&["file3.txt", "file4.txt"], &mut FailFast)

    // (Of course, if the receiver is not known to us we'd want to call check() anyway)
}
```

Working with std `Result`
-------------------------

`?` on a `Result` will Just Work™ if the error is a std `Error` type. This is because `Problem` implements `From<Error>`. In effect the `?` is the start of a causation chain.

That said, we want to put our best foot forward. Problemo comes with an extension trait for std `Result`, so we can insert a `via()` and/or a `with()` before the `?`. At the very least we can add a quick `via(common::LowLevelError)`.

The functions also have lazy versions, such as `map_via()` and `map_with()`, that will generate values only when there is an error.

Non-Static Errors
-----------------

Most errors are `'static`, as well as `Send` and `Sync`, because the expectation is that they will travel all the way to the caller across thread boundaries.

Some, however, aren't. If so, they cannot be captured in a `Problem`'s causation chain. This is famously the case with `PoisonError`, which we can get from locking a `Mutex`. We can, however, capture such an error's `Display` representation, which we enable with `into_message_problem()` (which uses the `common::MessageError` type mentioned above) as well as `into_thread_problem()` (which uses the `common::ThreadError` message error).

If we need to preserve more than the `Display` then we can use `map_into_problem()` to provide our own conversion function. It can make use of a new error type and/or add attachments.

FAQ
---

### Why should I use Problemo instead of the many alternatives?

Maybe because you appreciate its goals and design? We're not in competition with others and we're not trying to win you over. (Unless there were a trophy involved, then things would get heated!)

Error handling is a very opinionated topic and one size does not fit all. As it stands, *you're* going to have to do the homework to evaluate each library and decide which is "best".

At the very least we urge you to consider not only the experience of writing your code but also the experience of users of your code. How easy is it for them to inspect and handle the errors you return?

### Why doesn't `Problem` wrap a concrete error type? Wouldn't it be better to have the compiler to check for correct usage?

Our experience in large projects with error-handling libraries that require typed errors has led us to believe that it's a cumbersome and ultimately pointless requirement, especially for high-level functions that can have long causation chains.

In practice just having the compiler test that the topmost error is of the correct type guarantees very little if you're not actually handling the details of that error. It ends up just adding clutter and unnecessary complexity.

This is essentially a version of the long-running debate in the Java world about checked exceptions ([example](https://literatejava.com/exceptions/checked-exceptions-javas-biggest-mistake/)). It all hinges on the whether you believe that having the compiler enforce the recognition of an error type encourages programmers to handle that error. Do you? We don't.

### Why no pretty printing of causation chains with attachments?

This is deliberately out of scope because it's very specific to the *reason* for formatting, the reporting environment, and even personal taste. Are you printing out errors to a console for debugging? Are you displaying an error to a user in a dialog box? Are you logging errors to a textual file or a database for auditing? These are all very different use cases, some of which may require you to specifically hide sensitive information, provide text in multiple human languages, etc.

We might provide add-on libraries to help with this in the future, but we want to keep the core unopinionated in this regard.

### Why doesn't Problemo come with macros such as `bail!()`?

Most error-handling libraries have macros that optimize the creation and returning of errors so that it would take the smallest number of keystrokes. Problemo instead prefers verbosity, clarity, and debuggability by encouraging the use of explicit function calls.

We use macros only when functions *can't* work. Generally speaking our code is deliberately non-magical and should be easy for any Rust programmer to read and understand. Error handling is foundational and we believe that it should not be a mysterious black box.

You are of course free to create your own macros for Problemo but we don't want to promote them in our published API.

### I like Problemo but it's missing my favorite feature!

That's not a question! Anyway, we are happy to hear your suggestions. Please be nice about it and do keep in mind that we want to keep Problemo lean, simple, and focused on the essentials. If the feature is something that can be built on top of Problemo, perhaps as a supplementary library, then that will likely be the preferred route.

### Spanish?

Actually it's [Spanglish](https://en.wikipedia.org/wiki/No_problem#No_problemo). And even more actually it's [Esperanto](https://glosbe.com/eo/en/problemo). Mi havas naŭdek naŭ problemojn, sed hundino ne estas unu.

### "AI"?

Please, no.

Popular Alternatives
--------------------

* [error-stack](https://github.com/hashintel/hash/tree/main/libs/error-stack), like Problemo, supports chaining and attachments. It does, however, require you to provide a concrete error type for your returns, which it (confusingly in our opinion) calls the "context". It supports returning groups of errors as long as they are of that same "context" type. It also features backtraces and pretty printing.

* [rootcause](https://github.com/rootcause-rs/rootcause) works similarly to error-stack in practice while also supporting type-less wrappers, like Problemo. It also features first-class (but limited) support for non-static errors without having to convert them, which is achieved through an innovative use of generic markers. Its scope is broad and it's relatively complex. It includes a customizable error formatter and other powerful features.

* [anyhow](https://github.com/dtolnay/anyhow) is simple in its usage but is in fact a sophisticated library. It solves the problem of not being able to set the `source()` of a std `Error` by rewriting its dynamic dispatch vtable (wow!). It furthermore optimizes memory use by boxing into a narrow pointer. Anyhow lets you add non-errors to the causation chain via an internal wrapper, which it calls a "context". It only supports one attachment type, a backtrace, which is handled implicitly and automatically.

* [SNAFU](https://github.com/shepmaster/snafu) works similarly to Anyhow in practice but takes a different design approach by introducing its own set of traits as a replacement for std `Error` while also allowing for compatibility with it. This allows you to build custom, rich error types on top of SNAFU.

* [eyre](https://github.com/eyre-rs/eyre) is a fork of Anyhow with support for customizable formatting.

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](https://github.com/tliron/problemo/blob/main/LICENSE-APACHE)
* [MIT license](https://github.com/tliron/problemo/blob/main/LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
