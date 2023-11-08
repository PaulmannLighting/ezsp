use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Custom(Box<dyn std::error::Error>),
    ResidualBytes,
    UnexpectedEndOfStream,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Custom(error) => Display::fmt(error, f),
            Self::ResidualBytes => write!(f, "residual bytes"),
            Self::UnexpectedEndOfStream => write!(f, "unexpected end of stream"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Custom(error) => Some(error.as_ref()),
            Self::ResidualBytes | Self::UnexpectedEndOfStream => None,
        }
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Custom(msg.to_string().into())
    }
}
