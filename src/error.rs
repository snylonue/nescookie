use std::fmt::Display;

#[derive(Debug)]
pub enum ParseError {
    InvaildValue(String),
    TooFewFileds,
}
#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
    IoError(std::io::Error),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvaildValue(value) => write!(f, "InvalidValue: {}", value),
            Self::TooFewFileds => write!(f, "TooFewFields"),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Self::ParseError(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(e) => write!(f, "ParseError: {}", e),
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
