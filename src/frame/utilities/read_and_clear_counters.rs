use crate::counter::Counter;
use crate::frame::Parameters;
use anyhow::anyhow;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{empty, Empty, FlatMap};

pub const ID: u16 = 0x0065;
const TYPE_COUNT: usize = Counter::TypeCount as usize;

/// Retrieves and clears Ember counters.
///
/// See the [`Counter`] enumeration for the counter types.
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

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(_: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {})
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    values: [u16; TYPE_COUNT],
}

impl Response {
    #[must_use]
    pub const fn new(values: [u16; TYPE_COUNT]) -> Self {
        Self { values }
    }

    #[must_use]
    pub const fn values(&self) -> &[u16] {
        &self.values
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = FlatMap<IntoIter<u16, TYPE_COUNT>, [u8; 2], fn(u16) -> [u8; 2]>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter().flat_map(u16::to_be_bytes)
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer = [0; TYPE_COUNT * 2];
        src.read_exact(&mut buffer)?;
        let values: Vec<u16> = buffer
            .chunks_exact(2)
            .filter_map(|chunk| {
                if chunk.len() == 2 {
                    Some(u16::from_be_bytes([chunk[0], chunk[1]]))
                } else {
                    None
                }
            })
            .collect();
        Ok(Self {
            values: values
                .try_into()
                .map_err(|_| anyhow!("values size != {TYPE_COUNT}"))?,
        })
    }
}
