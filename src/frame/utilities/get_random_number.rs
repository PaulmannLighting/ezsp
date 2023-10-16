use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::status::{Misc, Status};
use never::Never;
use std::io::Read;

const ID: u16 = 0x0049;

/// Returns a pseudorandom number.
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
    status: Status,
    value: u16,
}

impl Response {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, status: Status, value: u16) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            status,
            value,
        }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    #[must_use]
    pub const fn value(&self) -> u16 {
        self.value
    }

    #[must_use]
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
        Some([self.status.into(), value_low, value_high])
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [status, value @ ..] = [0; 3];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            status: Status::try_from(status)?,
            value: u16::from_be_bytes(value),
        })
    }
}
