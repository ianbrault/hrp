/*
** src/err.rs
*/

use std::fmt;

pub type ResType<T> = Result<T, ErrType>;

pub enum ErrType {
    InvalidFormat(char),
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

impl fmt::Display for ErrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(ch)
                => write!(f, "invalid format character: \"{}\"", ch),
            Self::SodiumOxide(s)
                => write!(f, "libsodium {}", s),
        }
    }
}