use crate::ember::Status;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{empty, once, Empty, Once};

pub const ID: u16 = 0x0020;

/// Causes the stack to leave the current network.
///
/// This generates a stackStatusHandler callback to indicate that the network is down.
/// The radio will not be used until after sending a formNetwork or joinNetwork command.
#[derive(Debug, Eq, PartialEq)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Empty<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        empty()
    }
}

impl Readable for Command {
    fn try_read<R>(_: &mut R) -> Result<Self, crate::Error>
    where
        R: Read,
    {
        Ok(Self {})
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
