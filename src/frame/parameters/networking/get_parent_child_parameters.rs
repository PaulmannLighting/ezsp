use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{empty, once, Chain, Empty, Once};

pub const ID: u16 = 0x0029;

/// Returns information about the children of the local node and the parent of the local node.
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
    fn read_from<R>(_: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {})
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    child_count: u8,
    parent_eui64: u64,
    parent_node_id: u16,
}

impl Response {
    #[must_use]
    pub const fn new(child_count: u8, parent_eui64: u64, parent_node_id: u16) -> Self {
        Self {
            child_count,
            parent_eui64,
            parent_node_id,
        }
    }

    #[must_use]
    pub const fn child_count(&self) -> u8 {
        self.child_count
    }

    #[must_use]
    pub const fn parent_eui64(&self) -> u64 {
        self.parent_eui64
    }

    #[must_use]
    pub const fn parent_node_id(&self) -> u16 {
        self.parent_node_id
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter =
        Chain<Chain<Once<Self::Item>, IntoIter<Self::Item, 8>>, IntoIter<Self::Item, 2>>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.child_count)
            .chain(self.parent_eui64.to_be_bytes())
            .chain(self.parent_node_id.to_be_bytes())
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let child_count = src.read_num_be()?;
        let parent_eui64 = src.read_num_be()?;
        let parent_node_id = src.read_num_be()?;
        Ok(Self {
            child_count,
            parent_eui64,
            parent_node_id,
        })
    }
}
