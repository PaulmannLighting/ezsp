use crate::frame::header::Header;
use crate::frame::Frame;
use std::sync::Arc;

const ID: u16 = 0x0081;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    data: Arc<[u8]>,
}

impl Command {
    pub const fn new(header: Header, data: Arc<[u8]>) -> Self {
        Self { header, data }
    }

    pub const fn data_length(&self) -> u8 {
        self.data.len().try_into().expect("data length exceeds u8")
    }

    pub const fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Frame<ID> for Command {
    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Vec<u8> {
        let mut parameters = Vec::with_capacity(1 + self.data.len());
        parameters.push(self.data_length());
        parameters.extend_from_slice(&self.data);
        parameters
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    echo: Arc<[u8]>,
}

impl Response {
    pub const fn echo_length(&self) -> u8 {
        self.echo.len().try_into().expect("echo length exceeds u8")
    }

    pub const fn echo(&self) -> &[u8] {
        &self.echo
    }
}

impl Frame<ID> for Response {
    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Vec<u8> {
        let mut parameters = Vec::with_capacity(1 + self.echo.len());
        parameters.push(self.echo_length());
        parameters.extend_from_slice(&self.echo);
        parameters
    }
}
