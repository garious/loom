use std;
use core;

pub enum Error {
    IO(std::io::Error),
    NoSpace,
    ToLarge,
}

pub type Result<T> = core::result::Result<T, Error>;

impl core::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        return Error::IO(e);
    }
}
