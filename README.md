# hrp

`hrp` generates **H**uman-**R**eadable **P**asswords

`hrp` uses [libsodium](https://libsodium.gitbook.io/doc/) (through
[sodiumoxide](https://docs.rs/sodiumoxide/0.2.5/sodiumoxide/)) for
cryptographically-secure psudeorandom number generation (CSPRNG).

`hrp` selects words from a list of the 10,000 most common medium-length (5-8
characters) English words, as determined by the Google Trillion Word Corpus.

`hrp`'s default password format (`WWWDDDD`) results in 9,997,000,200,000,000
possible passwords.

## Usage

```
hrp 0.1.0
Ian Brault <ian.brault@engineering.ucla.edu>

hrp generates human-readable passwords.

USAGE: hrp [OPTIONS] [FORMAT]

OPTIONS:
    -h, --help       Prints help information
    -v, --version    Prints version information
```
