use crate::frame::header::Header;
use crate::frame::Frame;

const ID: u16 = 0x0057;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    new_pan: u16,
}

impl Command {
    pub const fn new(header: Header, new_pan: u16) -> Self {
        Self { header, new_pan }
    }

    pub const fn new_pan(&self) -> u16 {
        self.new_pan
    }
}

impl Frame<ID> for Command {
    type Parameters = [u8; 2];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Self::Parameters {
        self.new_pan.to_be_bytes()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: bool,
}

impl Response {
    pub const fn new(header: Header, status: bool) -> Self {
        Self { header, status }
    }

    pub const fn status(&self) -> bool {
        self.status
    }
}

impl Frame<ID> for Response {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Self::Parameters {
        [self.status.into()]
    }
}
