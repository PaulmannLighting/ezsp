use crate::ember::Status;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x0105;

/// Allows the Host to control the broadcast behaviour of a routing device used by the NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    config: u8,
    min_acks_needed: u8,
}

impl Command {
    #[must_use]
    pub const fn new(config: u8, min_acks_needed: u8) -> Self {
        Self {
            config,
            min_acks_needed,
        }
    }

    #[must_use]
    pub const fn config(&self) -> u8 {
        self.config
    }

    #[must_use]
    pub const fn min_acks_needed(&self) -> u8 {
        self.min_acks_needed
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Chain<Once<Self::Item>, Once<Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.config).chain(once(self.min_acks_needed))
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let [config, min_acks_needed] = src.read_array_exact()?;
        Ok(Self {
            config,
            min_acks_needed,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
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

impl Readable for Response {
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
