use crate::ember::types::PanId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Data {
    channel: u8,
    lqi: u8,
    rssi: i8,
    depth: u8,
    nwk_update_id: u8,
    power: i8,
    parent_priority: i8,
    pan_id: PanId,
    extended_pan_id: u64,
    sender: u16,
}

impl Data {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        channel: u8,
        lqi: u8,
        rssi: i8,
        depth: u8,
        nwk_update_id: u8,
        power: i8,
        parent_priority: i8,
        pan_id: PanId,
        extended_pan_id: u64,
        sender: u16,
    ) -> Self {
        Self {
            channel,
            lqi,
            rssi,
            depth,
            nwk_update_id,
            power,
            parent_priority,
            pan_id,
            extended_pan_id,
            sender,
        }
    }

    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    #[must_use]
    pub const fn lqi(&self) -> u8 {
        self.lqi
    }

    #[must_use]
    pub const fn rssi(&self) -> i8 {
        self.rssi
    }

    #[must_use]
    pub const fn depth(&self) -> u8 {
        self.depth
    }

    #[must_use]
    pub const fn nwk_update_id(&self) -> u8 {
        self.nwk_update_id
    }

    #[must_use]
    pub const fn power(&self) -> i8 {
        self.power
    }

    #[must_use]
    pub const fn parent_priority(&self) -> i8 {
        self.parent_priority
    }

    #[must_use]
    pub const fn pan_id(&self) -> PanId {
        self.pan_id
    }

    #[must_use]
    pub const fn extended_pan_id(&self) -> u64 {
        self.extended_pan_id
    }

    #[must_use]
    pub const fn sender(&self) -> u16 {
        self.sender
    }
}
