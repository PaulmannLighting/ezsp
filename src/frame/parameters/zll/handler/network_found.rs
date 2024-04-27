use le_stream::derive::FromLeBytes;

use crate::ember::zll::{DeviceInfoRecord, Network};
use crate::frame::Parameter;

const ID: u16 = 0x00B6;

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Handler {
    network_info: Network,
    is_device_info_null: bool,
    device_info: DeviceInfoRecord,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
}

impl Handler {
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

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
