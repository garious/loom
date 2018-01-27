use std;
use serde_json;
use core;
use crypto;
use crypto::symmetriccipher::SymmetricCipherError;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    JSON(serde_json::Error),
    AES(crypto::symmetriccipher::SymmetricCipherError),
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
impl core::convert::From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        return Error::JSON(e);
    }
}
impl core::convert::From<SymmetricCipherError> for Error {
    fn from(e: SymmetricCipherError) -> Error {
        return Error::AES(e);
    }
}

fn get_os_err<T>(r: Result<T>) -> Option<i32> {
    match r {
        Error::IO(e) => e.last_os_error(),
        _ => r 
    }
}

pub fn retry<F, T>(op: F) -> Result<T> 
    where F: Fn() -> Result<T>
{
	loop {
    	let ret = op();
		let c = get_os_err(ret);
		match c {
			Some(11) => (),
			_ => return ret,
		};
	}
}
