mod ashv2;

use crate::ezsp::Status;
use crate::types::ByteSizedVec;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Ashv2(::ashv2::Error),
    InvalidEzspStatus(u8),
    Custom(String),
}

impl From<::ashv2::Error> for Error {
    fn from(error: ::ashv2::Error) -> Self {
        Self::Ashv2(error)
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self::Custom(msg)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ashv2(error) => Display::fmt(error, f),
            Self::InvalidEzspStatus(status_code) => write!(f, "Invalid EZSP status: {status_code}"),
            Self::Custom(msg) => Display::fmt(msg, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Ashv2(error) => Some(error),
            _ => None,
        }
    }
}

pub trait Transport {
    async fn add_endpoint(
        &mut self,
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_clusters: ByteSizedVec<u16>,
        output_clusters: ByteSizedVec<u16>,
    ) -> Result<Status, Error>;
}
