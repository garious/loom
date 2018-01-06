use std;
use core;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    NoSpace,
    ToLarge,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match(self, other) {
            (&Error::NoSpace, &Error::NoSpace) => true,
            (&Error::ToLarge, &Error::ToLarge) => true,
            _ => false
        }
    }
    fn ne(&self, other: &Self) -> bool {
        (*self == *other) == false
    }
}

pub type Result<T> = core::result::Result<T, Error>;
impl core::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        return Error::IO(e);
    }
}
