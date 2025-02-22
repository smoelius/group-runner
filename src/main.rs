use std::{
    env::{args_os, var_os},
    ffi::OsStr,
    io::Error,
    path::Path,
    process::{Command, exit},
};

fn main() -> Result<(), Error> {
    if var_os("CI").is_none() {
        eprintln!("warning: environment variable `CI` is unset---are we running on GitHub?");
    }

    let args = args_os().collect::<Vec<_>>();

    if args.len() <= 1 {
        return Ok(());
    }

    let title = Path::new(&args[1]).file_name().map(OsStr::to_string_lossy);

    if let Some(title) = title.as_ref() {
        println!("::group::{title}");
    }

    let result = Command::new(&args[1]).args(&args[2..]).status();

    if title.is_some() {
        println!("::endgroup::");
    }

    let status = result?;

    if let Some(code) = status.code() {
        exit(code);
    }

    Ok(())
}
