use crate::types::EmberStatus;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x000A;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    token_id: u8,
}

impl Command {
    #[must_use]
    pub const fn new(token_id: u8) -> Self {
        Self { token_id }
    }

    #[must_use]
    pub const fn token_id(&self) -> u8 {
        self.token_id
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: EmberStatus,
    token_data: [u8; 8],
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, token_data: [u8; 8]) -> Self {
        Self { status, token_data }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn token_data(&self) -> [u8; 8] {
        self.token_data
    }
}
