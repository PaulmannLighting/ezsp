use crate::frame::header::Header;
use crate::frame::Frame;

const ID: u16 = 0x000D;

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    token_address: u16,
}

impl Response {
    pub const fn new(header: Header, token_address: u16) -> Self {
        Self {
            header,
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
