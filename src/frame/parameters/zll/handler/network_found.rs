use le_stream::FromLeStream;

use crate::ember::zll::{DeviceInfoRecord, Network};
use crate::frame::Parameter;

const ID: u16 = 0x00B6;

/// This call is fired when a ZLL network scan finds a ZLL network.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    network_info: Network,
    // Used to interpret deviceInfo field.
    is_device_info_null: bool,
    device_info: DeviceInfoRecord,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
}

impl Handler {
    /// Information about the network.
    #[must_use]
    pub const fn network_info(&self) -> &Network {
        &self.network_info
    }

    /// Device specific information.
    #[must_use]
    pub const fn device_info(&self) -> Option<&DeviceInfoRecord> {
        if self.is_device_info_null {
            None
        } else {
            Some(&self.device_info)
        }
    }

    /// The link quality from the node that last relayed the message.
    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }

    /// The energy level (in units of dBm) observed during reception.
    #[must_use]
    pub const fn last_hop_rssi(&self) -> i8 {
        self.last_hop_rssi
    }
}

impl Parameter for Handler {
    const ID: u16 = ID;
}
