use crate::ember::Status;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x0021;

/// The application may call this function when contact with the network has been lost.
///
/// The most common usage case is when an end device can no longer communicate with its parent
/// and wishes to find a new one. Another case is when a device has missed a Network Key update
/// and no longer has the current Network Key.
/// The stack will call ezspStackStatusHandler to indicate that the network is down,
/// then try to re-establish contact with the network by performing an active scan,
/// choosing a network with matching extended pan id, and sending a ZigBee network rejoin request.
/// A second call to the ezspStackStatusHandler callback indicates either the success or
/// the failure of the attempt.
/// The process takes approximately 150 milliseconds per channel to complete.
/// This call replaces the emberMobileNodeHasMoved API from EmberZNet 2.x,
/// which used MAC association and consequently took half a second longer to complete.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    have_current_network_key: bool,
    channel_mask: u32,
}

impl Command {
    #[must_use]
    pub const fn new(have_current_network_key: bool, channel_mask: u32) -> Self {
        Self {
            have_current_network_key,
            channel_mask,
        }
    }

    #[must_use]
    pub const fn have_current_network_key(&self) -> bool {
        self.have_current_network_key
    }

    #[must_use]
    pub const fn channel_mask(&self) -> u32 {
        self.channel_mask
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Chain<Once<Self::Item>, IntoIter<Self::Item, 4>>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.have_current_network_key.into()).chain(self.channel_mask.to_le_bytes())
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let have_current_network_key = src.read_bool()?;
        let channel_mask = src.read_num_le()?;
        Ok(Self {
            have_current_network_key,
            channel_mask,
        })
    }
}

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

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let status: u8 = src.read_num_le()?;
        Ok(Self {
            status: status.try_into()?,
        })
    }
}
