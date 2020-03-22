/*
** src/args.rs
*/

use crate::err::{ErrType, ResType};

use std::io::{self, Write};

const PROGRAM: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR:  &'static str = env!("CARGO_PKG_AUTHORS");

const HELP: &'static str = r#"
hrp generates human-readable passwords.

USAGE: hrp [OPTIONS] [FORMAT]

ARGS:
    <FORMAT>    Specifies the format of the generated password. Can be provided
                as a single string or multiple strings. Defaults to WWWDDDD.
                Acceptable format characters are:
                    W - word
                    D - digit

OPTIONS:
    -h, --help       Prints help information
    -v, --version    Prints version information
"#;

fn write_version() -> ResType<()> {
    writeln!(io::stdout(), "{} {}", PROGRAM, VERSION)?;
    Ok(())
}

fn write_help() -> ResType<()> {
    write_version()?;
    write!(io::stdout(), "{}\n{}", AUTHOR, HELP)?;
    Ok(())
}

pub fn parse(args: impl Iterator<Item=String>) -> ResType<String> {
    let mut fmt = String::new();

    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => {
                write_help()?;
                Err(ErrType::ArgExit)?;
            },
            "-v" | "--version" => {
                write_version()?;
                Err(ErrType::ArgExit)?;
            }
            _ => {
                fmt += arg.as_str();
            }
        }
    }

    // assign default format string if none was provided
    if fmt.is_empty() {
        fmt = "WWWDDDD".into();
    }

    Ok(fmt)
}