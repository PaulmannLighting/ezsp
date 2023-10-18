use crate::ezsp::network::scan::Type;
use crate::frame::Parameters;
use siliconlabs::Status;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x001A;
pub const ALL_CHANNELS: u32 = 0x07FF_F800;
pub const CURRENT_CHANNEL: u32 = 0x00;

/// This function will start a scan.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    scan_type: Type,
    channel_mask: u32,
    duration: u8,
}

impl Command {
    #[must_use]
    pub const fn new(scan_type: Type, channel_mask: u32, duration: u8) -> Self {
        Self {
            scan_type,
            channel_mask,
            duration,
        }
    }

    #[must_use]
    pub const fn all_channels(scan_type: Type, duration: u8) -> Self {
        Self::new(scan_type, ALL_CHANNELS, duration)
    }

    #[must_use]
    pub const fn current_channel(scan_type: Type, duration: u8) -> Self {
        Self::new(scan_type, CURRENT_CHANNEL, duration)
    }

    #[must_use]
    pub const fn scan_type(&self) -> Type {
        self.scan_type
    }

    #[must_use]
    pub const fn channel_mask(&self) -> u32 {
        self.channel_mask
    }

    #[must_use]
    pub const fn duration(&self) -> u8 {
        self.duration
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter =
        Chain<Chain<Once<Self::Item>, IntoIter<Self::Item, 4>>, IntoIter<Self::Item, 1>>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.scan_type.into())
            .chain(self.channel_mask.to_be_bytes())
            .chain(self.duration.to_be_bytes())
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [scan_type, channel_mask @ .., duration] = [0; 6];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            scan_type: Type::try_from(scan_type)?,
            channel_mask: u32::from_be_bytes(channel_mask),
            duration,
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
    type IntoIter = IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        u32::from(self.status).to_be_bytes().into_iter()
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer = [0; 4];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            status: Status::try_from(u32::from_be_bytes(buffer))?,
        })
    }
}
