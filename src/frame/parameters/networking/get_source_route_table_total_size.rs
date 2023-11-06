use crate::Error;
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
}

impl TryFrom<&[u8]> for Response {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() == 1 {
            Ok(Self {
                source_route_table_total_size: bytes[0],
            })
        } else {
            Err(Error::InvalidSize {
                expected: 1,
                found: bytes.len(),
            })
        }
    }
}
