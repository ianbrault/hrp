## hrp

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Crates.io](https://img.shields.io/crates/v/hrp.svg)](https://crates.io/crates/hrp)

`hrp` generates **H**uman-**R**eadable **P**asswords

`hrp` uses [libsodium](https://libsodium.gitbook.io/doc/) (through
[sodiumoxide](https://docs.rs/sodiumoxide/0.2.5/sodiumoxide/)) for
cryptographically-secure psudeorandom number generation (CSPRNG).

`hrp` selects words from a list of the 10,000 most common medium-length (5-8
characters) English words, as determined by the Google Trillion Word Corpus.

`hrp`'s default password format (`WWWDDDD`) results in 9,997,000,200,000,000
possible passwords.

### Usage

```
hrp 0.1.2
Ian Brault <ian.brault@engineering.ucla.edu>

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
```

### Installation

`hrp` can be installed with `cargo`

- Note that the binary file may be larger than expected, as it contains debug
symbols. To remove these, run `strip` on the binary

```
$ cargo install hrp
```

### Building

`hrp` can also be built from source

```
$ git clone https://github.com/ianbrault/hrp
$ cd hrp
$ cargo build --release
$ ./target/release/hrp --version
hrp 0.1.2
```
