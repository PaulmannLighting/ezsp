use crate::ember::Status;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x009C;

/// Sends a ZDO energy scan request.
///
/// This request may only be sent by the current network manager and must be unicast, not broadcast.
/// See ezsp-utils.h for related macros emberSetNetworkManagerRequest() and emberChangeChannelRequest().
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    target: u16,
    scan_channels: u32,
    scan_duration: u8,
    scan_count: u16,
}

impl Command {
    #[must_use]
    pub const fn new(target: u16, scan_channels: u32, scan_duration: u8, scan_count: u16) -> Self {
        Self {
            target,
            scan_channels,
            scan_duration,
            scan_count,
        }
    }

    #[must_use]
    pub const fn target(&self) -> u16 {
        self.target
    }

    #[must_use]
    pub const fn scan_channels(&self) -> u32 {
        self.scan_channels
    }

    #[must_use]
    pub const fn scan_duration(&self) -> u8 {
        self.scan_duration
    }

    #[must_use]
    pub const fn scan_count(&self) -> u16 {
        self.scan_count
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Chain<
        Chain<Chain<IntoIter<Self::Item, 2>, IntoIter<Self::Item, 4>>, IntoIter<Self::Item, 1>>,
        IntoIter<Self::Item, 2>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.target
            .to_le_bytes()
            .into_iter()
            .chain(self.scan_channels.to_le_bytes())
            .chain(self.scan_duration.to_le_bytes())
            .chain(self.scan_count.to_le_bytes())
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let target = src.read_num_le()?;
        let scan_channels = src.read_num_le()?;
        let scan_duration = src.read_num_le()?;
        let scan_count = src.read_num_le()?;
        Ok(Self {
            target,
            scan_channels,
            scan_duration,
            scan_count,
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
