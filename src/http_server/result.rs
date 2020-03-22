use rscam;
use std::convert;
use std::io;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    CameraError(rscam::Error),
    TcpError(io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

impl convert::From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::TcpError(error)
    }
}

impl convert::From<rscam::Error> for Error {
    fn from(error: rscam::Error) -> Self {
        Error::CameraError(error)
    }
}
