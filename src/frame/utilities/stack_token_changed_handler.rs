use crate::frame::header::{Control, Header};
use crate::frame::Frame;

const ID: u16 = 0x000D;

/// A callback invoked to inform the application that a stack token has changed.
#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    token_address: u16,
}

impl Response {
    pub const fn new(sequence: u8, control: Control, token_address: u16) -> Self {
        Self {
            header: Self::make_header(sequence, control),
            token_address,
        }
    }

    pub const fn token_address(&self) -> u16 {
        self.token_address
    }
}

impl Frame<ID> for Response {
    type Parameters = [u8; 2];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some(self.token_address.to_be_bytes())
    }
}
