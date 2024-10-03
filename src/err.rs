/*
** src/err.rs
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

use std::fmt;
use std::io;

pub type ResType<T> = Result<T, ErrType>;

pub enum ErrType {
    ArgExit,
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
            Self::ArgExit => write!(f, "-h or -v argument encountered"),
            Self::InvalidFormat(ch) => write!(f, "invalid format character: \"{}\"", ch),
            Self::IO(ioerr) => write!(f, "{}", ioerr),
            Self::SodiumOxide(s) => write!(f, "libsodium {}", s),
        }
    }
}
