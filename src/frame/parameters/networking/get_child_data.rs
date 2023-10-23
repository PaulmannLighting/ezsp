use crate::ember::child::Data;
use crate::ember::Status;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x004A;

/// Returns information about a child of the local node.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.index)
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {
            index: src.read_num_be()?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
    child_data: Data,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status, child_data: Data) -> Self {
        Self { status, child_data }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    #[must_use]
    pub const fn child_data(&self) -> &Data {
        &self.child_data
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Chain<Once<Self::Item>, <Data as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.status.into()).chain(self.child_data)
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let status: u8 = src.read_num_be()?;
        let child_data = Data::read_from(src)?;
        Ok(Self {
            status: status.try_into()?,
            child_data,
        })
    }
}
