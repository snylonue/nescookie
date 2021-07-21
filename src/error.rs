use crate::Rule;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    ParseError(pest::error::Error<Rule>),
    IoError(std::io::Error),
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(e: pest::error::Error<Rule>) -> Self {
        Self::ParseError(e)
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(e) => write!(f, "ParseError: {}", e),
            Self::IoError(e) => write!(f, "IoError: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ParseError(e) => Some(e),
            Self::IoError(e) => Some(e),
        }
    }
}
