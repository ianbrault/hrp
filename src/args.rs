/*
** src/args.rs
**
** Copyright (c) 2024 Ian Brault.
**
** This program is free software: you can redistribute it and/or modify
** it under the terms of the GNU General Public License as published by
** the Free Software Foundation, version 3.
**
** This program is distributed in the hope that it will be useful, but
** WITHOUT ANY WARRANTY; without even the implied warranty of
** MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
** General Public License for more details.
**
** You should have received a copy of the GNU General Public License
** along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

use crate::err::{ErrType, ResType};

use std::io::{self, Write};

const PROGRAM: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

const HELP: &str = r#"
hrp generates human-readable passwords.

USAGE: hrp [OPTIONS] [FORMAT]

ARGS:
    <FORMAT>    Specifies the format of the generated password. Can be provided
                as a single string or multiple strings. Defaults to WWWDDDD.
                Acceptable format characters are (case-insensitive):
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

pub fn parse(args: impl Iterator<Item = String>) -> ResType<String> {
    let mut fmt = String::new();

    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => {
                write_help()?;
                Err(ErrType::ArgExit)?;
            }
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
