use crate::ember::types::NodeId;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;

pub const ID: u16 = 0x0107;

/// Convert a node ID to a child index.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    child_id: NodeId,
}

impl Command {
    #[must_use]
    pub const fn new(child_id: NodeId) -> Self {
        Self { child_id }
    }

    #[must_use]
    pub const fn child_id(&self) -> NodeId {
        self.child_id
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<u8, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.child_id.to_le_bytes().into_iter()
    }
}

impl Readable for Command {
    fn try_read<R>(src: &mut R) -> Result<Self, crate::Error>
    where
        R: Read,
    {
        Ok(Self {
            child_id: src.read_num_le()?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    child_index: u8,
}

impl Response {
    #[must_use]
    pub const fn new(child_index: u8) -> Self {
        Self { child_index }
    }

    #[must_use]
    pub const fn child_index(&self) -> u8 {
        self.child_index
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = IntoIter<u8, 1>;

    fn into_iter(self) -> Self::IntoIter {
        self.child_index.to_le_bytes().into_iter()
    }
}
