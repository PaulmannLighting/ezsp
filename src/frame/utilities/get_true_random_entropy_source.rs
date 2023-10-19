use crate::entropy_source::EntropySource;
use crate::frame::Parameters;
use crate::util::ReadExt;
use std::io::Read;
use std::iter::{empty, once, Empty, Once};

pub const ID: u16 = 0x00FC;

/// Returns the entropy source used for true random number generation.
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
    entropy_source: EntropySource,
}

impl Response {
    #[must_use]
    pub const fn new(entropy_source: EntropySource) -> Self {
        Self { entropy_source }
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.entropy_source.into())
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let entropy_source: u8 = src.read_num_be()?;
        Ok(Self {
            entropy_source: entropy_source.try_into()?,
        })
    }
}
