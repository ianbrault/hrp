/*
** src/main.rs
*/

mod args;
mod err;

use crate::err::{ErrType, ResType};

use colored::Colorize;
use lazy_static::lazy_static;
use sodiumoxide::randombytes::randombytes_uniform;

use std::env;
use std::fs;
use std::io::{self, Write};

/*
** statically embed the word list and indices into the binary
*/

static WORDS: &'static str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/share/google-10000-english-usa-no-swears-medium.txt"));

lazy_static! {
    static ref INDICES: Vec<usize> = {
        let indices_file = concat!(env!("OUT_DIR"), "/indices.txt");
        fs::read_to_string(indices_file).unwrap()
            .split("\n")
            .map(|s| s.parse::<usize>().unwrap())
            .collect()
    };
}

fn get_word() -> String {
    // get a random index into the index list
    let i = randombytes_uniform(INDICES.len() as u32);

    // slice the word out of the full string
    let lb = INDICES[i as usize];
    let ub = INDICES[i as usize + 1] - 1;
    String::from(&WORDS[lb..ub])
}

fn get_digit() -> u8 {
    randombytes_uniform(10) as u8
}

fn main_inner() -> ResType<()> {
    // initialize libsodium
    sodiumoxide::init()
        .or_else(|_| Err(ErrType::sodiumoxide_init_error()))?;

    // parse command-line arguments, return the format string
    let fmt = args::parse(env::args().skip(1))?;

    // track generated words to ensure uniqueness
    let mut words = Vec::with_capacity(8);

    for f in fmt.chars() {
        match f {
            'w' | 'W' => {
                let mut word = get_word();
                // ensure word is unique
                while words.iter().find(|&w| *w == word).is_some() {
                    word = get_word();
                }
                write!(io::stdout(), "{}", word)?;
                words.push(word);
            },
            'd' | 'D' => {
                write!(io::stdout(), "{}", get_digit())?;
            },
            _   => return Err(ErrType::invalid_format_char(f)),
        }
    }
    writeln!(io::stdout(), "")?;
    Ok(())
}

fn main() {
    match main_inner() {
        Err(ErrType::ArgExit) => (),
        Err(err) => eprintln!("{} {}", "error:".bold().red(), err),
        _ => (),
    }
}
