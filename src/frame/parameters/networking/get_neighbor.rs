use crate::ember::Status;
use std::array::IntoIter;

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

pub struct Response {
    status: Status,
    value: NeighborTableEntry,
}
