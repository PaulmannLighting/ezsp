use crate::config;
use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::status::Status;
use std::io::Read;

const ID: u16 = 0x0053;

/// Writes a configuration value to the NCP.
///
/// Configuration values can be modified by the Host after the NCP has reset.
/// Once the status of the stack changes to EMBER_NETWORK_UP,
/// configuration values can no longer be modified and this command
/// will respond with [`Status::Error`]`(`[`Error::InvalidCall`][crate::status::Error::InvalidCall]`)`.
#[derive(Debug)]
pub struct Command {
    header: Header,
    config_id: config::Id,
    value: u16,
}

impl Command {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, config_id: config::Id, value: u16) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            config_id,
            value,
        }
    }

    #[must_use]
    pub const fn config_id(&self) -> config::Id {
        self.config_id
    }

    #[must_use]
    pub const fn value(&self) -> u16 {
        self.value
    }
}

impl Frame<ID> for Command {
    type Parameters = [u8; 3];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let [value_low, value_high] = self.value.to_be_bytes();
        Some([self.config_id.into(), value_low, value_high])
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [config_id, value @ ..]: [u8; 3] = [0; 3];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            config_id: config::Id::try_from(config_id)?,
            value: u16::from_be_bytes(value),
        })
    }
}

#[derive(Debug)]
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
    pub const fn status(&self) -> Status {
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
            status: Status::try_from(status)?,
        })
    }
}
