use std::error;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result;
use std::string::FromUtf8Error;

use serde::{de, ser};


/// This enum is storing all possible errors that can occur when serializing
/// or deserializing a value using BERT.
#[derive(Debug)]
pub enum Error {
    /// A custom error provided by serde occured.
    Custom(String),
    /// The data source contains not enough bytes to parse a value.
    EndOfStream,
    /// Some IO error occured when processing a value.
    Io(io::Error),
    /// Some error occured while converting a string.
    FromUtf8(FromUtf8Error),
    /// Passed tag is invalid or not supported.
    InvalidTag,
    /// Passed type of value is not supported by BERT.
    UnsupportedType,
    /// Version number has not specified or invalid.
    InvalidVersionNumber,
    /// The data source contains trailing bytes after all values were read.
    TrailingBytes,
}


impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Custom(ref string) => string,
            Error::EndOfStream => "unexpected end of file",
            Error::FromUtf8(ref error) => error.description(),
            Error::Io(ref error) => StdError::description(error),
            Error::InvalidTag => "invalid tag",
            Error::UnsupportedType => "type is not supported by BERT",
            Error::InvalidVersionNumber => {
                "version number has not specified or invalid"
            }
            Error::TrailingBytes => "unexpected trailing bytes",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref error) => Some(error),
            Error::FromUtf8(ref error) => Some(error),
            _ => None,
        }
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Custom(ref s) => write!(f, "custom error: {}", s),
            _ => f.write_str(self.description()),
        }
    }
}


impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}


impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Error {
        Error::FromUtf8(error)
    }
}


impl ser::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Error {
        Error::Custom(msg.into())
    }
}


impl de::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Error {
        Error::Custom(msg.into())
    }

    fn end_of_stream() -> Error {
        Error::EndOfStream
    }
}


/// Helper alias for `Result` objects that return a BERT `Error`.
pub type Result<T> = result::Result<T, Error>;
