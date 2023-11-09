use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberStatus};

pub const ID: u16 = 0x0009;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    token_id: u8,
    token_data: [u8; 8],
}

impl Command {
    #[must_use]
    pub const fn new(token_id: u8, token_data: [u8; 8]) -> Self {
        Self { token_id, token_data }
    }

    #[must_use]
    pub const fn token_id(&self) -> u8 {
        self.token_id
    }


    #[must_use]
    pub const fn token_data(&self) -> [u8; 8] {
        self.token_data
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
