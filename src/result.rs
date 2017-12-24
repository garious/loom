use std;

enum Error {
    IO(std::io::Error),
    NoSpace,
    ToLarge,
}

type Result<T> = Result<T, Error>;

pub fn fromIO<T>(r: std::io::Result<T>) -> Result<T> {
    match {
        Ok(t) => Ok(t),
        Err(e) => Err(Error::IO(e)),
    }
}
