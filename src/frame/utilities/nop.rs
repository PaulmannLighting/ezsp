use crate::frame::header::Header;
use crate::frame::Frame;

const ID: u16 = 0x0005;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
}

impl Command {
    pub const fn new(header: Header) -> Self {
        Self { header }
    }
}

impl Frame<ID> for Command {
    type Parameters = [u8; 0];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Self::Parameters {
        []
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
}

impl Response {
    pub const fn new(header: Header) -> Self {
        Self { header }
    }
}

impl Frame<ID> for Response {
    type Parameters = [u8; 0];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Self::Parameters {
        []
    }
}
