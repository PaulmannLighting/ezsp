use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{sl_status_t,EzspNetworkScanType};

pub const ID: u16 = 0x001A;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    scan_type: EzspNetworkScanType,
    channel_mask: u32,
    duration: u8,
}

impl Command {
    #[must_use]
    pub const fn new(scan_type: EzspNetworkScanType, channel_mask: u32, duration: u8) -> Self {
        Self { scan_type, channel_mask, duration }
    }

    #[must_use]
    pub const fn scan_type(&self) -> EzspNetworkScanType {
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

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    status: sl_status_t,
}

impl Response {
    #[must_use]
    pub const fn new(status: sl_status_t) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> sl_status_t {
        self.status
    }
}
