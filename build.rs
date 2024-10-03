/*
** build.rs
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

use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let base_dir = env::var("CARGO_MANIFEST_DIR")?;
    let out_dir = env::var("OUT_DIR")?;

    //
    // generate indices for the dictionary words
    //

    let in_file = Path::new(&base_dir)
        .join("share")
        .join("google-10000-english-usa-no-swears-medium.txt");
    let out_file = Path::new(&out_dir).join("indices.txt");

    let words = fs::read_to_string(&in_file)?;
    let mut acc = String::with_capacity(10000);
    acc.push('0');

    // word indices are 1 past each newline
    let idxs = words
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '\n')
        .map(|(i, _)| i + 1)
        // convert index list into a single string
        .map(|i| i.to_string())
        .fold(acc, |acc, s| acc + "\n" + s.as_str());

    // write indices to the output file
    fs::write(out_file, idxs)?;

    Ok(())
}
