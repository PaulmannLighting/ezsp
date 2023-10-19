use crate::ezsp::Status;
use crate::frame::Parameters;
use crate::util::ReadExt;
use std::io::Read;
use std::iter::{once, Once};

pub const ID: u16 = 0x0058;

/// Indicates that the NCP received an invalid command.
#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    reason: Status,
}

impl Response {
    #[must_use]
    pub const fn new(reason: Status) -> Self {
        Self { reason }
    }

    #[must_use]
    pub const fn reason(&self) -> Status {
        self.reason
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.reason.into())
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {
            reason: src.read_u8()?.try_into()?,
        })
    }
}
