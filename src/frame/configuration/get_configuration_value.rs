use crate::config;
use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::status::Status;
use num_traits::ToPrimitive;
use std::io::Read;

const ID: u16 = 0x0052;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    config_id: config::Id,
}

/// Reads a configuration value from the NCP.
impl Command {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, config_id: config::Id) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            config_id,
        }
    }

    #[must_use]
    pub const fn config_id(&self) -> config::Id {
        self.config_id
    }
}

impl Frame<ID> for Command {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self
            .config_id
            .to_u8()
            .expect("could not convert config id to u8")])
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [config_id] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            config_id: config::Id::try_from(config_id)?,
        })
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
    pub const fn status(&self) -> &Status {
        &self.status
    }

    #[must_use]
    pub const fn value(&self) -> u16 {
        self.value
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
