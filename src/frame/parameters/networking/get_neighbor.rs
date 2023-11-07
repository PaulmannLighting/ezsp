use crate::ember::neighbor::TableEntry;
use crate::ember::Status;
use crate::read_write::Readable;
use crate::Error;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x0079;

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
    type IntoIter = IntoIter<u8, 1>;

    fn into_iter(self) -> Self::IntoIter {
        self.index.to_le_bytes().into_iter()
    }
}

impl Readable for Command {
    fn try_read<R>(src: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(Self::new(src.read_num_le()?))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
    value: TableEntry,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status, value: TableEntry) -> Self {
        Self { status, value }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    #[must_use]
    pub const fn value(&self) -> &TableEntry {
        &self.value
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Chain<
        Once<Self::Item>,
        Chain<
            Chain<
                Chain<
                    Chain<
                        Chain<IntoIter<Self::Item, 2>, IntoIter<Self::Item, 1>>,
                        IntoIter<Self::Item, 1>,
                    >,
                    IntoIter<Self::Item, 1>,
                >,
                IntoIter<Self::Item, 1>,
            >,
            IntoIter<Self::Item, 8>,
        >,
    >;

    fn into_iter(self) -> Self::IntoIter {
        once(self.status.into()).chain(self.value)
    }
}

impl Readable for Response {
    fn try_read<R>(src: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let status: u8 = src.read_num_le()?;
        let value = TableEntry::try_read(src)?;
        Ok(Self {
            status: status.try_into()?,
            value,
        })
    }
}
