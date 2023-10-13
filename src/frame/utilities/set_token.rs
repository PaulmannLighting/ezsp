use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::status::Status;
use num_traits::ToPrimitive;

const ID: u16 = 0x0009;

/// Sets a token (8 bytes of non-volatile storage) in the Simulated EEPROM of the NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    token_id: u8,
    token_data: [u8; 8],
}

impl Command {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, token_id: u8, token_data: [u8; 8]) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            token_id,
            token_data,
        }
    }

    #[must_use]
    pub const fn token_id(&self) -> u8 {
        self.token_id
    }

    #[must_use]
    pub const fn token_data(&self) -> &[u8] {
        &self.token_data
    }
}

impl Frame<ID> for Command {
    type Parameters = [u8; 9];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([
            self.token_id,
            self.token_data[0],
            self.token_data[1],
            self.token_data[2],
            self.token_data[3],
            self.token_data[4],
            self.token_data[5],
            self.token_data[6],
            self.token_data[7],
        ])
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, status: Status) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            status,
        }
    }

    #[must_use]
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
