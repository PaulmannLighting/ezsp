use crate::frame::header::Header;
use crate::frame::Frame;
use crate::status::Status;
use num_traits::ToPrimitive;

const ID: u16 = 0x0105;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    config: u8,
    min_acks_needed: u8,
}

impl Command {
    pub const fn new(header: Header, config: u8, min_acks_needed: u8) -> Self {
        Self {
            header,
            config,
            min_acks_needed,
        }
    }

    pub const fn config(&self) -> u8 {
        self.config
    }

    pub const fn min_acks_needed(&self) -> u8 {
        self.min_acks_needed
    }
}

impl Frame<ID> for Command {
    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Vec<u8> {
        vec![self.config, self.min_acks_needed]
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: Status,
}

impl Response {
    pub const fn new(header: Header, status: Status) -> Self {
        Self { header, status }
    }

    pub const fn status(&self) -> &Status {
        &self.status
    }
}

impl Frame<ID> for Response {
    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Vec<u8> {
        vec![self.status.to_u8().expect("could not convert status to u8")]
    }
}
