use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ParserError {
    InvalidArguments,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::InvalidArguments => "Invalid CLI Arguments",
        })
    }
}

impl Error for ParserError {}