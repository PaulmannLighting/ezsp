use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Read(le_stream::Error),
    Write(Box<dyn std::error::Error>),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(error) => write!(f, "Read error: {error}"),
            Self::Write(error) => write!(f, "Write error: {error}"),
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Self::Read(error) => Some(error),
            Self::Write(error) => error.source(),
        }
    }
}

impl From<le_stream::Error> for Error {
    fn from(error: le_stream::Error) -> Self {
        Self::Read(error)
    }
}
