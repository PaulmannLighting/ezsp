use crate::ember::NULL_NODE_ID;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Once};

pub const ID: u16 = 0x0106;

/// Convert a child index to a node ID.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    child_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(child_index: u8) -> Self {
        Self { child_index }
    }

    #[must_use]
    pub const fn child_index(&self) -> u8 {
        self.child_index
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.child_index)
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {
            child_index: src.read_num_le()?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    child_id: u16,
}

impl Response {
    #[must_use]
    pub const fn new(child_id: u16) -> Self {
        Self { child_id }
    }

    #[must_use]
    pub const fn child_id(&self) -> u16 {
        self.child_id
    }

    #[must_use]
    pub const fn effective_child_id(&self) -> Option<u16> {
        if self.child_id == NULL_NODE_ID {
            None
        } else {
            Some(self.child_id)
        }
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.child_id.to_le_bytes().into_iter()
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {
            child_id: src.read_num_le()?,
        })
    }
}
