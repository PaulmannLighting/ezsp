use crate::ember::network::init::Bitmask;
use crate::ember::Status;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
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
    network_init_bitmask: Bitmask,
}

impl Command {
    #[must_use]
    pub const fn new(network_init_bitmask: Bitmask) -> Self {
        Self {
            network_init_bitmask,
        }
    }

    #[must_use]
    pub const fn network_init_bitmask(&self) -> Bitmask {
        self.network_init_bitmask
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        <Bitmask as Into<u16>>::into(self.network_init_bitmask)
            .to_le_bytes()
            .into_iter()
    }
}

impl Readable for Command {
    fn try_read<R>(src: &mut R) -> Result<Self, crate::Error>
    where
        R: Read,
    {
        let network_init_bitmask: u16 = src.read_num_le()?;
        Ok(Self {
            network_init_bitmask: network_init_bitmask.try_into()?,
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
    fn try_read<R>(src: &mut R) -> Result<Self, crate::Error>
    where
        R: Read,
    {
        let status: u8 = src.read_num_le()?;
        Ok(Self {
            status: status.try_into()?,
        })
    }
}
