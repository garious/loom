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
    AddrParse(std::net::AddrParseError),
    NoSpace,
    ToLarge,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Error::NoSpace, &Error::NoSpace) => true,
            (&Error::ToLarge, &Error::ToLarge) => true,
            _ => false,
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

impl core::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IO(e)
    }
}
impl core::convert::From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::JSON(e)
    }
}
impl core::convert::From<SymmetricCipherError> for Error {
    fn from(e: SymmetricCipherError) -> Error {
        Error::AES(e)
    }
}
impl core::convert::From<std::net::AddrParseError> for Error {
    fn from(e: std::net::AddrParseError) -> Error {
        Error::AddrParse(e)
    }
}
//pub fn retry<F, T>(r: &mut Result<T>,  op: F) -> ()
//    where F: Fn() -> Result<T>
//{
//	loop {
//    	*r = op();
//		let c = match r {
//            &mut Err(Error::IO(e)) => e.raw_os_error(),
//            _ => None
//        };
//		match c {
//			Some(11) => (),
//			_ => return ()
//		};
//	}
//}
