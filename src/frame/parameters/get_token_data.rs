use crate::ember::token::Data;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0102;

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
    status: u8,
    token_data: Data,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, token_data: Data) -> Self {
        Self {
            status: status.into(),
            token_data,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn token_data(&self) -> &Data {
        &self.token_data
    }
}
