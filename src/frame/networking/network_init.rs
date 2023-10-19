use crate::ember::Status;
use crate::frame::Parameters;
use crate::network::InitBitmask;
use crate::util::ReadExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Once};

pub const ID: u16 = 0x0017;

/// Resume network operation after a reboot.
///
/// The node retains its original type.
/// This should be called on startup whether or not the node was previously part of a network.
/// [`Status::NotJoined`] is returned if the node is not part of a network.
/// This command accepts options to control the network initialization.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    network_init_bitmask: InitBitmask,
}

impl Command {
    #[must_use]
    pub const fn new(network_init_bitmask: InitBitmask) -> Self {
        Self {
            network_init_bitmask,
        }
    }

    #[must_use]
    pub const fn network_init_bitmask(&self) -> InitBitmask {
        self.network_init_bitmask
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        <InitBitmask as Into<u16>>::into(self.network_init_bitmask)
            .to_be_bytes()
            .into_iter()
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {
            network_init_bitmask: src.read_u16_be()?.try_into()?,
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

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {
            status: src.read_u8()?.try_into()?,
        })
    }
}
