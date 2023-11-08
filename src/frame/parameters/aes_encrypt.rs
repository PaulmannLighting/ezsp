use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0094;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    plaintext: [u8; 16],
    key: [u8; 16],
}

impl Command {
    #[must_use]
    pub const fn new(plaintext: [u8; 16], key: [u8; 16]) -> Self {
        Self { plaintext, key }
    }

    #[must_use]
    pub const fn plaintext(&self) -> [u8; 16] {
        self.plaintext
    }

    #[must_use]
    pub const fn key(&self) -> [u8; 16] {
        self.key
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    ciphertext: [u8; 16],
}

impl Response {
    #[must_use]
    pub const fn new(ciphertext: [u8; 16]) -> Self {
        Self { ciphertext }
    }

    #[must_use]
    pub const fn ciphertext(&self) -> [u8; 16] {
        self.ciphertext
    }
}
