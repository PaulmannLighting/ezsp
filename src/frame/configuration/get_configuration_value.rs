use crate::config;
use crate::frame::header::Header;
use crate::frame::Frame;
use crate::status::Status;
use num_traits::ToPrimitive;

const ID: u16 = 0x0052;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    config_id: config::Id,
}

impl Command {
    pub const fn new(header: Header, config_id: config::Id) -> Self {
        Self { header, config_id }
    }

    pub const fn config_id(&self) -> &config::Id {
        &self.config_id
    }
}

impl Frame<ID> for Command {
    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Vec<u8> {
        vec![self
            .config_id
            .to_u8()
            .expect("could not convert config id to u8")]
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: Status,
    value: u16,
}

impl Response {
    pub const fn new(header: Header, status: Status, value: u16) -> Self {
        Self {
            header,
            status,
            value,
        }
    }

    pub const fn status(&self) -> &Status {
        &self.status
    }

    pub const fn value(&self) -> u16 {
        self.value
    }
}

impl Frame<ID> for Response {
    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Vec<u8> {
        let mut parameters = vec![self.status.to_u8().expect("could not convert status to u8")];
        parameters.extend_from_slice(&self.value.to_be_bytes());
        parameters
    }
}
