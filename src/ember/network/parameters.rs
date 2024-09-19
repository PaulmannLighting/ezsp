use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::join::Method;
use crate::ember::types::PanId;

/// Network parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
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
    /// Create new network parameters.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        extended_pan_id: u64,
        pan_id: PanId,
        radio_tx_power: u8,
        radio_channel: u8,
        join_method: Method,
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

    /// Return the network's extended PAN identifier.
    #[must_use]
    pub const fn extended_pan_id(&self) -> u64 {
        self.extended_pan_id
    }

    /// Return the network's PAN identifier.
    #[must_use]
    pub const fn pan_id(&self) -> PanId {
        self.pan_id
    }

    /// Return the power setting in dBm.
    #[must_use]
    pub const fn radio_tx_power(&self) -> u8 {
        self.radio_tx_power
    }

    /// Return the radio channel.
    #[must_use]
    pub const fn radio_channel(&self) -> u8 {
        self.radio_channel
    }

    /// Return the method used to initially join the network.
    #[must_use]
    pub fn join_method(&self) -> Option<Method> {
        Method::from_u8(self.join_method)
    }

    /// Return the NWK Manager ID.
    ///
    /// The ID of the network manager in the current network.
    /// This may only be set at joining when using `EMBER_USE_CONFIGURED_NWK_STATE` as the join method.
    #[must_use]
    pub const fn nwk_manager_id(&self) -> u16 {
        self.nwk_manager_id
    }

    /// Return the NWK Update ID.
    ///
    /// The value of the ZigBee nwkUpdateId known by the stack.
    /// This is used to determine the newest instance of the network after a PAN ID or channel change.
    /// This may only be set at joining when using `EMBER_USE_CONFIGURED_NWK_STATE` as the join method.
    #[must_use]
    pub const fn nwk_update_id(&self) -> u8 {
        self.nwk_update_id
    }

    /// Return the NWK channel mask.
    ///
    /// The list of preferred channels that the NWK manager has told this device to use when
    /// searching for the network.
    /// This may only be set at joining when using `EMBER_USE_CONFIGURED_NWK_STATE` as the join method.
    #[must_use]
    pub const fn channels(&self) -> u32 {
        self.channels
    }
}
