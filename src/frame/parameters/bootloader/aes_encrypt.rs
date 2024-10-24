//! Parameters for the [`Bootloader::aes_encrypt()`](crate::Bootloader::aes_encrypt) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Identified;

const ID: u16 = 0x0094;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    plaintext: [u8; 16],
    key: [u8; 16],
}

impl Command {
    #[must_use]
    pub const fn new(plaintext: [u8; 16], key: [u8; 16]) -> Self {
        Self { plaintext, key }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    ciphertext: [u8; 16],
}

impl Response {
    /// Returns the ciphertext.
    #[must_use]
    pub const fn ciphertext(&self) -> [u8; 16] {
        self.ciphertext
    }
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
