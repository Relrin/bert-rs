use std::error;
use std::fmt;
use std::io;
use std::result;

use serde::{de, ser};


/// This enum is storing all possible errors that can occur when serializing or
/// deserializing a value using BERT.
#[derive(Debug)]
pub enum Error {
    Custom(String),
    EndOfStream,
    Io(io::Error),
    InvalidTag,
    UnsupportedType,
}


impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Custom(_) => "syntax error",
            Error::EndOfStream => "EOF while parsing a binary stream",
            Error::Io(ref err) => err.description(),
            Error::InvalidTag => "invalid tag",
            Error::UnsupportedType => "unsupported type",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            _ => None,
        }
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Custom(ref msg) => write!(f, "{}", msg),
            Error::EndOfStream => write!(f, "EOF while parsing a stream"),
            Error::Io(ref err) => err.fmt(f),
            Error::InvalidTag => write!(f, "Invalid tag"),
            Error::UnsupportedType => {
                write!(f, "Type is not supported by BERT")
            }
        }
    }
}


impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}


impl ser::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Self {
        Error::Custom(msg.into())
    }
}


impl de::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Self {
        Error::Custom(msg.into())
    }

    fn end_of_stream() -> Self {
        Error::EndOfStream
    }
}


/// Helper alias for `Result` objects that return a BERT `Error`.
pub type Result<T> = result::Result<T, Error>;
