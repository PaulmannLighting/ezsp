use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::status::Status;
use num_traits::ToPrimitive;

const ID: u16 = 0x0105;

/// Allows the Host to control the broadcast behaviour of a routing device used by the NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    config: u8,
    min_acks_needed: u8,
}

impl Command {
    pub const fn new(sequence: u8, control: Control, config: u8, min_acks_needed: u8) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
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
    type Parameters = [u8; 2];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.config, self.min_acks_needed])
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: Status,
}

impl Response {
    pub const fn new(sequence: u8, control: Control, status: Status) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            status,
        }
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
