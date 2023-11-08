use crate::types::{bool, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0012;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    binary_message: bool,
    message_length: u8,
    message_contents: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(
        binary_message: bool,
        message_length: u8,
        message_contents: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            binary_message,
            message_length,
            message_contents,
        }
    }

    #[must_use]
    pub const fn binary_message(&self) -> bool {
        self.binary_message
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
