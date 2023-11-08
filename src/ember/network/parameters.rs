use crate::ember::join_method::JoinMethod;
use crate::ember::types::PanId;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Parameters {
    extended_pan_id: u64,
    pan_id: PanId,
    radio_tx_power: u8,
    radio_channel: u8,
    join_method: u8,
    nwk_manager_id: u16,
    nwk_update_id: u8,
    channels: u32,
}

impl Parameters {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        extended_pan_id: u64,
        pan_id: PanId,
        radio_tx_power: u8,
        radio_channel: u8,
        join_method: JoinMethod,
        nwk_manager_id: u16,
        nwk_update_id: u8,
        channels: u32,
    ) -> Self {
        Self {
            extended_pan_id,
            pan_id,
            radio_tx_power,
            radio_channel,
            join_method: join_method.into(),
            nwk_manager_id,
            nwk_update_id,
            channels,
        }
    }

    #[must_use]
    pub const fn extended_pan_id(&self) -> u64 {
        self.extended_pan_id
    }

    #[must_use]
    pub const fn pan_id(&self) -> PanId {
        self.pan_id
    }

    #[must_use]
    pub const fn radio_tx_power(&self) -> u8 {
        self.radio_tx_power
    }

    #[must_use]
    pub const fn radio_channel(&self) -> u8 {
        self.radio_channel
    }

    #[must_use]
    pub fn join_method(&self) -> Option<JoinMethod> {
        JoinMethod::from_u8(self.join_method)
    }

    #[must_use]
    pub const fn nwk_manager_id(&self) -> u16 {
        self.nwk_manager_id
    }

    #[must_use]
    pub const fn nwk_update_id(&self) -> u8 {
        self.nwk_update_id
    }

    #[must_use]
    pub const fn channels(&self) -> u32 {
        self.channels
    }
}
