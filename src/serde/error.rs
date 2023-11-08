use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Custom(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Custom(string) => write!(f, "{string}"),
        }
    }
}

impl std::error::Error for Error {}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Custom(msg.to_string())
    }
}
