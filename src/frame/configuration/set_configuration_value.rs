use crate::config::Id;
use crate::ezsp::Status;
use crate::frame::Parameters;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x0053;

/// Writes a configuration value to the NCP.
///
/// Configuration values can be modified by the Host after the NCP has reset.
/// Once the status of the stack changes to [`crate::ember::Status::NetworkUp`],
/// configuration values can no longer be modified and this command will respond with
/// [`Status::Error`]`(`[`Error::InvalidCall`][crate::ezsp::Error::InvalidCall]`)`.
#[derive(Debug)]
pub struct Command {
    config_id: Id,
    value: u16,
}

impl Command {
    #[must_use]
    pub const fn new(config_id: Id, value: u16) -> Self {
        Self { config_id, value }
    }

    #[must_use]
    pub const fn config_id(&self) -> Id {
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
        let config_id: u8 = src.read_num_be()?;
        let value = src.read_num_be()?;
        Ok(Self {
            config_id: config_id.try_into()?,
            value,
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
        let status: u8 = src.read_num_be()?;
        Ok(Self {
            status: status.try_into()?,
        })
    }
}
