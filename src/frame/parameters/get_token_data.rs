use crate::types::{EmberStatus, EmberTokenData};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0102;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    token: u32,
    index: u32,
}

impl Command {
    #[must_use]
    pub const fn new(token: u32, index: u32) -> Self {
        Self { token, index }
    }

    #[must_use]
    pub const fn token(&self) -> u32 {
        self.token
    }

    #[must_use]
    pub const fn index(&self) -> u32 {
        self.index
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: EmberStatus,
    token_data: EmberTokenData,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, token_data: EmberTokenData) -> Self {
        Self { status, token_data }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn token_data(&self) -> EmberTokenData {
        self.token_data
    }
}
