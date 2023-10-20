use crate::ember::{network, Status};
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Once};

pub const ID: u16 = 0x001E;

/// Forms a new network by becoming the coordinator.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    parameters: network::Parameters,
}

impl Command {
    #[must_use]
    pub const fn new(parameters: network::Parameters) -> Self {
        Self { parameters }
    }

    #[must_use]
    pub const fn parameters(&self) -> &network::Parameters {
        &self.parameters
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = <network::Parameters as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.parameters.into_iter()
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        network::Parameters::read_from(src).map(Self::new)
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
