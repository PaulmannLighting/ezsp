use crate::ember::Status;
use crate::frame::Parameters;
use crate::util::ReadExt;
use std::io::Read;
use std::iter::{once, Once};

pub const ID: u16 = 0x0019;

/// A callback invoked when the status of the stack changes.
///
/// If the status parameter equals EMBER_NETWORK_UP,
/// then the getNetworkParameters command can be called to obtain the new network parameters.
/// If any of the parameters are being stored in nonvolatile memory by the Host,
/// the stored values should be updated.
#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.status.into())
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let status: u8 = src.read_num_be()?;
        Ok(Self {
            status: status.try_into()?,
        })
    }
}
