use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use std::io::Read;

const ID: u16 = 0x0057;

/// Triggers a pan id update message.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    new_pan: u16,
}

impl Command {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, new_pan: u16) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            new_pan,
        }
    }

    #[must_use]
    pub const fn new_pan(&self) -> u16 {
        self.new_pan
    }
}

impl Frame<ID> for Command {
    type Parameters = [u8; 2];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some(self.new_pan.to_be_bytes())
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer: [u8; 2] = [0; 2];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            new_pan: u16::from_be_bytes(buffer),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: bool,
}

impl Response {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, status: bool) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            status,
        }
    }

    #[must_use]
    pub const fn status(&self) -> bool {
        self.status
    }
}

impl Frame<ID> for Response {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.status.into()])
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [status]: [u8; 1] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            status: status != 0,
        })
    }
}
