use crate::ember::Status;
use crate::frame::Parameters;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x001C;

/// Returns the status of the current scan of type [`crate::ezsp::network::scan::Type`].
///
/// [`Status::Success`] signals that the scan has completed.
/// Other error conditions signify a failure to scan on the channel specified.
#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    channel: u8,
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(channel: u8, status: Status) -> Self {
        Self { channel, status }
    }

    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Chain<Once<Self::Item>, Once<Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.channel).chain(once(self.status.into()))
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let [channel, status] = src.read_array_exact()?;
        Ok(Self {
            channel,
            status: status.try_into()?,
        })
    }
}
