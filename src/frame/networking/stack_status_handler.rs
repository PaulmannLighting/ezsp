use crate::ember::Status;
use crate::frame::Parameters;
use std::array::IntoIter;
use std::io::Read;

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
    type IntoIter = IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        [self.status.into()].into_iter()
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [status] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            status: Status::try_from(status)?,
        })
    }
}
