use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0054;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    payload_length: u8,
    payload: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub const fn new(payload_length: u8, payload: ByteSizedVec<u8>) -> Self {
        Self {
            payload_length,
            payload,
        }
    }

    #[must_use]
    pub const fn payload_length(&self) -> u8 {
        self.payload_length
    }

    #[must_use]
    pub const fn payload(&self) -> ByteSizedVec<u8> {
        self.payload
    }
}
