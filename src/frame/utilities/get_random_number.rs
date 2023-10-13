use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::status::{Misc, Status};
use never::Never;
use num_traits::ToPrimitive;

const ID: u16 = 0x0049;

/// Returns a pseudorandom number.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
}

impl Command {
    pub const fn new(sequence: u8, control: Control) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
        }
    }
}

impl Frame<ID> for Command {
    type Parameters = Never;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        None
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: Status,
    value: u16,
}

impl Response {
    pub const fn new(sequence: u8, control: Control, status: Status, value: u16) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
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

    pub fn succeeded(&self) -> bool {
        self.status == Status::Misc(Misc::Success)
    }
}

impl Frame<ID> for Response {
    type Parameters = [u8; 3];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let [value_low, value_high] = self.value.to_be_bytes();
        Some([
            self.status.to_u8().expect("could not convert status to u8"),
            value_low,
            value_high,
        ])
    }
}
