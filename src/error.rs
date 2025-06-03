use std::error;
use std::fmt;

#[derive(Debug)]
pub enum DecodingError {
    ParsingError(String),
    IOError(std::io::Error),
}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::ParsingError(ref err) => write!(f, "Parsing error: {}", err),
            Self::IOError(ref err) => write!(f, "IO error: {}", err),
        }
    }
}

impl error::Error for DecodingError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::ParsingError(_) => None,
            Self::IOError(ref err) => Some(err),
        }
    }
}

impl From<std::io::Error> for DecodingError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

#[derive(Debug)]
pub enum EncodingError {
    ParsingError(String),
    DeserializeError(serde_json::Error),
    IOError(std::io::Error),
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::ParsingError(ref err) => write!(f, "Parsing error: {}", err),
            Self::DeserializeError(ref err) => write!(f, "Deserialize error: {}", err),
            Self::IOError(ref err) => write!(f, "IO error: {}", err),
        }
    }
}

impl error::Error for EncodingError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::ParsingError(_) => None,
            Self::DeserializeError(ref err) => Some(err),
            Self::IOError(ref err) => Some(err),
        }
    }
}

impl From<std::io::Error> for EncodingError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<serde_json::Error> for EncodingError {
    fn from(value: serde_json::Error) -> Self {
        Self::DeserializeError(value)
    }
}
