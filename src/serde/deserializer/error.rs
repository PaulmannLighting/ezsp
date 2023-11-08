use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Custom(String),
    Eof,
    NotSelfDescribing,
    ResidualBytes,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Custom(string) => write!(f, "{string}"),
            Self::Eof => write!(f, "end of file"),
            Self::NotSelfDescribing => write!(f, "byte streams are not self-describing"),
            Self::ResidualBytes => write!(f, "residual bytes"),
        }
    }
}

impl std::error::Error for Error {}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Custom(msg.to_string())
    }
}
