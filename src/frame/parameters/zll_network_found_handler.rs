use crate::ember::zll::{DeviceInfoRecord, Network};
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00B6;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    network_info: Network,
    is_device_info_null: bool,
    device_info: DeviceInfoRecord,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
}

impl Response {
    #[must_use]
    pub const fn new(
        network_info: Network,
        is_device_info_null: bool,
        device_info: DeviceInfoRecord,
        last_hop_lqi: u8,
        last_hop_rssi: i8,
    ) -> Self {
        Self {
            network_info,
            is_device_info_null,
            device_info,
            last_hop_lqi,
            last_hop_rssi,
        }
    }

    #[must_use]
    pub const fn network_info(&self) -> &Network {
        &self.network_info
    }

    #[must_use]
    pub const fn is_device_info_null(&self) -> bool {
        self.is_device_info_null
    }

    #[must_use]
    pub const fn device_info(&self) -> &DeviceInfoRecord {
        &self.device_info
    }

    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }

    #[must_use]
    pub const fn last_hop_rssi(&self) -> i8 {
        self.last_hop_rssi
    }
}
