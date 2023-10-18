use std::array::IntoIter;
use std::iter::{empty, Empty};

pub const ID: u16 = 0x00FC;

/// Returns the 16-bit node ID of the local node.
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

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    interface_count: u8,
}

impl Response {
    #[must_use]
    pub const fn new(interface_count: u8) -> Self {
        Self { interface_count }
    }

    #[must_use]
    pub const fn interface_count(&self) -> u8 {
        self.interface_count
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        self.interface_count.to_be_bytes().into_iter()
    }
}
