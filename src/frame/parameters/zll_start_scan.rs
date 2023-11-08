use crate::types::{int8_t, EmberNodeType, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00B4;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    channel_mask: u32,
    radio_power_for_scan: int8_t,
    node_type: EmberNodeType,
}

impl Command {
    #[must_use]
    pub const fn new(
        channel_mask: u32,
        radio_power_for_scan: int8_t,
        node_type: EmberNodeType,
    ) -> Self {
        Self {
            channel_mask,
            radio_power_for_scan,
            node_type,
        }
    }

    #[must_use]
    pub const fn channel_mask(&self) -> u32 {
        self.channel_mask
    }

    #[must_use]
    pub const fn radio_power_for_scan(&self) -> int8_t {
        self.radio_power_for_scan
    }

    #[must_use]
    pub const fn node_type(&self) -> EmberNodeType {
        self.node_type
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
