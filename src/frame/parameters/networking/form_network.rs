use crate::ember::network::Parameters;
use crate::ember::Status;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Once};

pub const ID: u16 = 0x001E;

/// Forms a new network by becoming the coordinator.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    parameters: Parameters,
}

impl Command {
    #[must_use]
    pub const fn new(parameters: Parameters) -> Self {
        Self { parameters }
    }

    #[must_use]
    pub const fn parameters(&self) -> &Parameters {
        &self.parameters
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = <Parameters as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.parameters.into_iter()
    }
}

impl Readable for Command {
    fn try_read<R>(src: &mut R) -> Result<Self, crate::Error>
    where
        R: Read,
    {
        Parameters::try_read(src).map(Self::new)
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
    fn try_read<R>(src: &mut R) -> Result<Self, crate::Error>
    where
        R: Read,
    {
        let status: u8 = src.read_num_le()?;
        Ok(Self {
            status: status.try_into()?,
        })
    }
}
