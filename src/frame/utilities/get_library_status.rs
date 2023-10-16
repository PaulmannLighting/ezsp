use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use std::io::Read;

const ID: u16 = 0x0001;

/// This retrieves the status of the passed library ID to determine if it is compiled into the stack.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    library_id: u8,
}

impl Command {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, library_id: u8) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            library_id,
        }
    }

    #[must_use]
    pub const fn library_id(&self) -> u8 {
        self.library_id
    }
}

impl Frame<ID> for Command {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.library_id])
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [library_id] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self { header, library_id })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: u8,
}

impl Response {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, status: u8) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            status,
        }
    }

    #[must_use]
    pub const fn status(&self) -> u8 {
        self.status
    }
}

impl Frame<ID> for Response {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.status])
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [status] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self { header, status })
    }
}
