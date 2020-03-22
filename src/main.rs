/*
** src/main.rs
*/

mod err;

use crate::err::{ErrType, ResType};

use colored::Colorize;
use lazy_static::lazy_static;
use sodiumoxide::randombytes::randombytes_uniform;

use std::env;
use std::fs;

/*
** statically embed the word list and indices into the binary
*/

static WORDS: &'static str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/share/google-10000-english-usa-no-swears-medium.txt"));

lazy_static! {
    static ref INDICES: Vec<usize> = {
        let indices_file = concat!(
            env!("CARGO_MANIFEST_DIR"), "/share/indices.txt");
        fs::read_to_string(indices_file).unwrap()
            .split("\n")
            .map(|s| s.parse::<usize>().unwrap())
            .collect()
    };
}

fn _parse_args(_args: impl Iterator<Item=String>) -> String {
    unimplemented!()
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
    // FIXME: unimplemented
    // let fmt = parse_args(env::args().skip(1));
    let fmt = "WWWDDDD";

    // track generated words to ensure uniqueness
    let mut words: Vec<String> = vec![];

    for f in fmt.chars() {
        match f {
            'W' => {
                let mut word = get_word();
                // ensure word is unique
                while words.iter().find(|&w| *w == word).is_some() {
                    word = get_word();
                }
                print!("{}", word);
                words.push(word);
            },
            'D' => {
                print!("{}", get_digit());
            },
            _   => return Err(ErrType::invalid_format_char(f)),
        }
    }
    println!();
    Ok(())
}

fn main() {
    match main_inner() {
        Err(err) => println!("{} {}", "error:".red(), err),
        _ => (),
    }
}
