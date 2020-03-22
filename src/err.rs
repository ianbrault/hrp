/*
** src/err.rs
*/

use std::fmt;
use std::io;

pub type ResType<T> = Result<T, ErrType>;

pub enum ErrType {
    InvalidFormat(char),
    IO(io::Error),
    SodiumOxide(String),
}

impl ErrType {
    pub fn sodiumoxide_init_error() -> Self {
        Self::SodiumOxide(String::from("failed to initialize"))
    }

    pub fn invalid_format_char(ch: char) -> Self {
        Self::InvalidFormat(ch)
    }
}

impl From<io::Error> for ErrType {
    fn from(ioerr: io::Error) -> Self {
        Self::IO(ioerr)
    }
}

impl fmt::Display for ErrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(ch)
                => write!(f, "invalid format character: \"{}\"", ch),
            Self::IO(ioerr)
                => write!(f, "{}", ioerr),
            Self::SodiumOxide(s)
                => write!(f, "libsodium {}", s),
        }
    }
}