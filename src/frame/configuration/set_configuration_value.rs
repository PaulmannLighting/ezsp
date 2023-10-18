use crate::config;
use crate::ezsp::Status;
use crate::frame::Parameters;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x0053;

/// Writes a configuration value to the NCP.
///
/// Configuration values can be modified by the Host after the NCP has reset.
/// Once the status of the stack changes to EMBER_NETWORK_UP,
/// configuration values can no longer be modified and this command
/// will respond with [`Status::Error`]`(`[`Error::InvalidCall`][crate::ezsp::Error::InvalidCall]`)`.
#[derive(Debug)]
pub struct Command {
    config_id: config::Id,
    value: u16,
}

impl Command {
    #[must_use]
    pub const fn new(config_id: config::Id, value: u16) -> Self {
        Self { config_id, value }
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

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Chain<Once<Self::Item>, IntoIter<Self::Item, 2>>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.config_id.into()).chain(self.value.to_be_bytes())
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [config_id, value @ ..] = [0; 3];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            config_id: config::Id::try_from(config_id)?,
            value: u16::from_be_bytes(value),
        })
    }
}

#[derive(Debug)]
pub struct Response {
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.status.into())
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [status] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            status: Status::try_from(status)?,
        })
    }
}
