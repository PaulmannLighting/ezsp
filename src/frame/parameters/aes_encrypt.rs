use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0094;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    plaintext: [u8; 16],
    key: [u8; 16],
}

impl Command {
    #[must_use]
    pub const fn new(plaintext: [u8; 16], key: [u8; 16]) -> Self {
        Self { plaintext, key }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    ciphertext: [u8; 16],
}

impl Response {
    #[must_use]
    pub const fn ciphertext(&self) -> [u8; 16] {
        self.ciphertext
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: u16 = ID;
}
