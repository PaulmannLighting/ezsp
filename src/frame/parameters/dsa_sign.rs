use crate::types::EmberStatus;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00A6;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    message_length: u8,
    message_contents: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(message_length: u8, message_contents: ByteSizedVec<u8>) -> Self {
        Self {
            message_length,
            message_contents,
        }
    }

    #[must_use]
    pub const fn message_length(&self) -> u8 {
        self.message_length
    }

    #[must_use]
    pub const fn message_contents(&self) -> ByteSizedVec<u8> {
        self.message_contents
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
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
