use crate::ember::Status;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Once};

pub const ID: u16 = 0x0022;

/// Tells the stack to allow other nodes to join the network with this node as their parent.
///
/// Joining is initially disabled by default.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    duration: u8,
}

impl Command {
    #[must_use]
    pub const fn new(duration: u8) -> Self {
        Self { duration }
    }

    #[must_use]
    pub const fn duration(&self) -> u8 {
        self.duration
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.duration)
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {
            duration: src.read_num_be()?,
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
