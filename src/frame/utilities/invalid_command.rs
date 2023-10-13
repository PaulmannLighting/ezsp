use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::status::Status;
use num_traits::ToPrimitive;

const ID: u16 = 0x0058;

/// Indicates that the NCP received an invalid command.
#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    reason: Status,
}

impl Response {
    pub const fn new(sequence: u8, control: Control, reason: Status) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            reason,
        }
    }

    pub const fn reason(&self) -> &Status {
        &self.reason
    }
}

impl Frame<ID> for Response {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.reason.to_u8().expect("could not convert reason to u8")])
    }
}
