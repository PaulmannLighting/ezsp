use crate::types::{int8_t, EmberMultiPhyNwkConfig, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00F8;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    phy_index: u8,
    page: u8,
    channel: u8,
    power: int8_t,
    bitmask: EmberMultiPhyNwkConfig,
}

impl Command {
    #[must_use]
    pub const fn new(
        phy_index: u8,
        page: u8,
        channel: u8,
        power: int8_t,
        bitmask: EmberMultiPhyNwkConfig,
    ) -> Self {
        Self {
            phy_index,
            page,
            channel,
            power,
            bitmask,
        }
    }

    #[must_use]
    pub const fn phy_index(&self) -> u8 {
        self.phy_index
    }

    #[must_use]
    pub const fn page(&self) -> u8 {
        self.page
    }

    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    #[must_use]
    pub const fn power(&self) -> int8_t {
        self.power
    }

    #[must_use]
    pub const fn bitmask(&self) -> EmberMultiPhyNwkConfig {
        self.bitmask
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
