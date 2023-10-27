use crate::ember::child::Data;
use crate::ember::Status;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x00AC;

/// Sets child data to the child table token.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    index: u8,
    child_data: Data,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8, child_data: Data) -> Self {
        Self { index, child_data }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    #[must_use]
    pub const fn child_data(&self) -> &Data {
        &self.child_data
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Chain<Once<Self::Item>, <Data as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.index).chain(self.child_data)
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let index = src.read_num_le()?;
        let child_data = Data::read_from(src)?;
        Ok(Self { index, child_data })
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
        let status: u8 = src.read_num_le()?;
        Ok(Self {
            status: status.try_into()?,
        })
    }
}
