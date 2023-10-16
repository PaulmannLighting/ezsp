use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use never::Never;
use std::io::Read;

const ID: u16 = 0x0005;

/// Allows the Host to control the broadcast behaviour of a routing device used by the NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
}

impl Command {
    #[must_use]
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

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Self::read_header(src).map(|header| Self { header })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
}

impl Response {
    #[must_use]
    pub const fn new(sequence: u8, control: Control) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
        }
    }
}

impl Frame<ID> for Response {
    type Parameters = Never;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        None
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Self::read_header(src).map(|header| Self { header })
    }
}
