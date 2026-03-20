use std::error;
use std::fmt;
use std::io;
use std::num::ParseFloatError;
use std::result;
use std::string::FromUtf8Error;

use serde::{de, ser};


/// This enum is storing all possible errors that can occur when serializing
/// or deserializing a value using BERT
#[derive(Debug)]
pub enum Error {
    /// A custom error provided by serde occurred.
    Custom(String),
    /// The data source contains not enough bytes to parse a value.
    EndOfStream,
    /// Some IO error occurred when processing a value.
    Io(io::Error),
    /// Some error occurred while converting a string.
    FromUtf8(FromUtf8Error),
    /// Invalid float for a value encoded as string
    InvalidFloat(ParseFloatError),
    /// Passed tag is invalid or not supported.
    InvalidTag,
    /// Passed type of value is not supported by BERT.
    UnsupportedType,
    /// Version number has not specified or invalid.
    InvalidVersionNumber,
    /// The data source contains trailing bytes after all values were read.
    TrailingBytes,
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Custom(ref s) => write!(f, "custom error: {}", s),
            Error::EndOfStream => f.write_str("unexpected end of stream"),
            Error::FromUtf8(ref error) => write!(f, "{}", error),
            Error::Io(ref error) => write!(f, "{}", error),
            Error::InvalidTag => f.write_str("invalid tag"),
            Error::InvalidFloat(ref value) => {
                write!(f, "Invalid float '{}'.", value)
            },
            Error::UnsupportedType => f.write_str("type is not supported by BERT"),
            Error::InvalidVersionNumber => {
                f.write_str("version number has not specified or invalid")
            }
            Error::TrailingBytes => f.write_str("unexpected trailing bytes"),
        }
    }
}


impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Io(ref error) => Some(error),
            Error::FromUtf8(ref error) => Some(error),
            _ => None,
        }
    }
}


impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}


impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Error {
        Error::FromUtf8(error)
    }
}


impl From<ParseFloatError> for Error {
    fn from(error: ParseFloatError) -> Error {
        Error::InvalidFloat(error)
    }
}


impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}


impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}


/// Helper alias for `Result` objects that return a BERT `Error`
pub type Result<T> = result::Result<T, Error>;
