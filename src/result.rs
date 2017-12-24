use std;
use core;

pub enum Error {
    IO(std::io::Error),
    NoSpace,
    ToLarge,
}

pub type Result<T> = core::result::Result<T, Error>;

pub fn fromIO<T>(r: std::io::Result<T>) -> Result<T> {
    match r {
        Ok(t) => Ok(t),
        Err(e) => Err(Error::IO(e)),
    }
}
