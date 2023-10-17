use crate::frame::Parameters;
use crate::status::Status;
use std::array::IntoIter;
use std::io::Read;

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
    type IntoIter = IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        [self.reason.into()].into_iter()
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [reason] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            reason: Status::try_from(reason)?,
        })
    }
}
