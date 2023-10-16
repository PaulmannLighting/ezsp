use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use never::Never;
use std::io::Read;

const ID: u16 = 0x009D;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    delay: u16,
}

impl Command {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, delay: u16) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            delay,
        }
    }

    #[must_use]
    pub const fn delay(&self) -> u16 {
        self.delay
    }
}

impl Frame<ID> for Command {
    type Parameters = [u8; 2];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some(self.delay.to_be_bytes())
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer = [0; 2];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            delay: u16::from_be_bytes(buffer),
        })
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
