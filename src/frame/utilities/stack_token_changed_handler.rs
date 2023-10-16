use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use std::io::Read;

const ID: u16 = 0x000D;

/// A callback invoked to inform the application that a stack token has changed.
#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    token_address: u16,
}

impl Response {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, token_address: u16) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            token_address,
        }
    }

    #[must_use]
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

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer: [u8; 2] = [0; 2];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            token_address: u16::from_be_bytes(buffer),
        })
    }
}
