use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::status::Status;
use std::io::Read;

const ID: u16 = 0x0058;

/// Indicates that the NCP received an invalid command.
#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    reason: Status,
}

impl Response {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, reason: Status) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            reason,
        }
    }

    #[must_use]
    pub const fn reason(&self) -> Status {
        self.reason
    }
}

impl Frame<ID> for Response {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.reason.into()])
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [reason] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            reason: Status::try_from(reason)?,
        })
    }
}
