mod ashv2;

use crate::ember::key::Data;
use crate::ember::Eui64;
use crate::types::ByteSizedVec;
use crate::{ember, ezsp};
use le_stream::ToLeBytes;
use std::fmt::{Display, Formatter};
use std::future::Future;

#[derive(Debug)]
pub enum Error {
    Ashv2(::ashv2::Error),
    InvalidEzspStatus(u8),
    InvalidEmberStatus(u8),
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
            Self::InvalidEzspStatus(status) => write!(f, "Invalid EZSP status: {status}"),
            Self::InvalidEmberStatus(status) => write!(f, "Invalid Ember status: {status}"),
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
    fn next_command<T>(&mut self, frame_id: u16, parameters: T) -> Vec<u8>
    where
        T: ToLeBytes;

    fn add_endpoint(
        &mut self,
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_clusters: ByteSizedVec<u16>,
        output_clusters: ByteSizedVec<u16>,
    ) -> impl Future<Output = Result<ezsp::Status, Error>>;

    fn add_or_update_key_table_entry(
        &mut self,
        address: Eui64,
        link_key: bool,
        key_data: Data,
    ) -> impl Future<Output = Result<ember::Status, Error>>;
}
