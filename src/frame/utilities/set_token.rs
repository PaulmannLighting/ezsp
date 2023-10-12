use crate::frame::header::Header;
use crate::frame::Frame;
use crate::status::Status;
use num_traits::ToPrimitive;
use std::sync::Arc;

const ID: u16 = 0x0009;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    token_id: u8,
    token_data: Arc<[u8]>,
}

impl Command {
    pub const fn new(header: Header, token_id: u8, token_data: Arc<[u8]>) -> Self {
        Self {
            header,
            token_id,
            token_data,
        }
    }

    pub const fn token_id(&self) -> u8 {
        self.token_id
    }

    pub const fn token_data(&self) -> &[u8] {
        &self.token_data
    }
}

impl Frame<ID> for Command {
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(1 + self.token_data.len());
        parameters.push(self.token_id);
        parameters.extend_from_slice(&self.token_data);
        Some(parameters)
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
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.status.to_u8().expect("could not convert status to u8")])
    }
}
