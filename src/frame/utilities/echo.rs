use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use std::sync::Arc;

const ID: u16 = 0x0081;

/// Variable length data from the Host is echoed back by the NCP.
///
/// This command has no other effects and is designed
/// for testing the link between the Host and NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    data: Arc<[u8]>,
}

impl Command {
    pub const fn new(sequence: u8, control: Control, data: Arc<[u8]>) -> Self {
        Self {
            header: Self::make_header(sequence, control),
            data,
        }
    }

    pub const fn data_length(&self) -> u8 {
        self.data.len().try_into().expect("data length exceeds u8")
    }

    pub const fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Frame<ID> for Command {
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(1 + self.data.len());
        parameters.push(self.data_length());
        parameters.extend_from_slice(&self.data);
        Some(parameters)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    echo: Arc<[u8]>,
}

impl Response {
    pub const fn new(sequence: u8, control: Control, echo: Arc<[u8]>) -> Self {
        Self {
            header: Self::make_header(sequence, control),
            echo,
        }
    }

    pub const fn echo_length(&self) -> u8 {
        self.echo.len().try_into().expect("echo length exceeds u8")
    }

    pub const fn echo(&self) -> &[u8] {
        &self.echo
    }
}

impl Frame<ID> for Response {
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(1 + self.echo.len());
        parameters.push(self.echo_length());
        parameters.extend_from_slice(&self.echo);
        Some(parameters)
    }
}
