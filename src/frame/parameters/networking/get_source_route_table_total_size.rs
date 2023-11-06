use crate::read_write::Readable;
use crate::Error;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{empty, Empty};

pub const ID: u16 = 0x00C3;

#[derive(Debug, Eq, PartialEq)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Default for Command {
    fn default() -> Self {
        Self::new()
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
    source_route_table_total_size: u8,
}

impl Response {
    #[must_use]
    pub const fn new(source_route_table_total_size: u8) -> Self {
        Self {
            source_route_table_total_size,
        }
    }

    #[must_use]
    pub const fn source_route_table_total_size(&self) -> u8 {
        self.source_route_table_total_size
    }
}

impl Readable for Response {
    fn try_read<R>(src: &mut R) -> Result<Self, crate::Error>
    where
        R: Read,
    {
        Ok(Self::new(src.read_num_le()?))
    }
}
