use {
    derive_more::*,
    problemo::{common::*, *},
    std::{io, sync::*},
};

pub fn main() -> Result<(), Problem> {
    if let Err(problem) = do_some_io() {
        for cause in &problem.causes {
            println!("error: {:?}", cause.error);
        }

        if problem.has_type::<io::Error>() {
            println!();
            println!("has i/o!");
        }

        if problem.has_type::<LowLevelErrors>() {
            println!();
            println!("has low-level!");
        }

        if let Some(cause) = problem.cause_of_type::<LowLevelErrors>() {
            println!();
            println!("has low-level!");
            for cause in cause.iter_under() {
                println!("  because: {}", cause.error);
            }
        }

        if problem.has(&LowLevelErrors::IO) {
            println!();
            println!("has low-level i/o!");
        }

        println!();
        for attachment in problem.attachments_of_type::<String>() {
            println!("string attachment: {}", attachment);
        }

        if let Some(attachment) = problem.attachment_of_type::<String>() {
            println!();
            println!("first string attachment: {}", attachment);
        }

        for backtrace in problem.attachments_of_type::<backtrace::Backtrace>() {
            println!();
            print!("backtrace:\n{:?}", backtrace);
        }
    }

    let mut problems = Problems::default();
    let _strings = read_files(&["file1.txt", "file2.txt"], &mut problems)?;
    problems.check()?;

    Ok(())
}

#[derive(Debug, Display, Error, PartialEq, Eq)]
enum LowLevelErrors {
    #[display("low-level I/O")]
    IO,
    //Concurrency,
}

fn read_files<ProblemReceiverT>(
    paths: &[&str],
    problems: &mut ProblemReceiverT,
) -> Result<Vec<String>, Problem>
where
    ProblemReceiverT: ProblemReceiver,
{
    let mut strings = Vec::default();
    for path in paths {
        if let Some(string) = std::fs::read_to_string(path)
            .via(common::LowLevelError)
            .give_ok(problems)?
        {
            strings.push(string);
        }
    }
    Ok(strings)
}

fn do_some_io() -> Result<String, Problem> {
    hello(true)?;

    let locked = Mutex::new(100);
    let _locked = locked.lock().into_thread_problem()?;

    std::fs::read_to_string("")
        .into_problem()
        .with("att3".to_string())
        .via(LowLevelErrors::IO)
        .with("att1".to_string())
}

// fn _do_some_io2() -> Result<String, Problem> {
//     std::fs::read_to_string("").into_problem()
// }

fn hello(ok: bool) -> Result<String, Problem> {
    if ok {
        Ok("hello".into())
    } else {
        Err("hello failed"
            .into_message_problem()
            .with("att2".to_string()))
    }
}
