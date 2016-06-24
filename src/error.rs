extern crate openssl;

use std::io::{self};
use std::result;
use std::fmt::{self};
use std::error::Error as StdError;

use openssl::ssl::error::SslError;

pub type Result<T> = result::Result<T, Error>;

/// A set of errors that can occur in the IMAP client
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Ssl(SslError),
    BadResponse(Vec<String>),
    NoResponse(Vec<String>),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<SslError> for Error {
    fn from(err: SslError) -> Error {
        Error::Ssl(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => fmt::Display::fmt(e, f),
            Error::Ssl(ref e) => fmt::Display::fmt(e, f),
            ref e => f.write_str(e.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref e) => e.description(),
            Error::Ssl(ref e) => e.description(),
            Error::BadResponse(_) => "Bad Response",
            Error::NoResponse(_) => "No Response"
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref e) => Some(e),
            Error::Ssl(ref e) => Some(e),
            _ => None,
        }
    }
}