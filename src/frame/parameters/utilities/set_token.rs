use crate::ember::Status;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0009;

#[derive(Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Command {
    token_id: u8,
    token_data: [u8; 8],
}

impl Command {
    #[must_use]
    pub const fn new(token_id: u8, token_data: [u8; 8]) -> Self {
        Self {
            token_id,
            token_data,
        }
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

#[derive(Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
