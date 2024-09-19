use crate::ember::types::PanId;
use le_stream::derive::{FromLeStream, ToLeStream};

/// The parameters of a ZigBee network.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Network {
    channel: u8,
    pan_id: PanId,
    extended_pan_id: u64,
    allowing_join: bool,
    stack_profile: u8,
    nwk_update_id: u8,
}

impl Network {
    /// Create a new Zigbee network.
    #[must_use]
    pub const fn new(
        channel: u8,
        pan_id: PanId,
        extended_pan_id: u64,
        allowing_join: bool,
        stack_profile: u8,
        nwk_update_id: u8,
    ) -> Self {
        Self {
            channel,
            pan_id,
            extended_pan_id,
            allowing_join,
            stack_profile,
            nwk_update_id,
        }
    }

    /// Return the 802.15.4 channel associated with the network.
    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    /// Return the network's PAN identifier.
    #[must_use]
    pub const fn pan_id(&self) -> PanId {
        self.pan_id
    }

    /// Return the network's extended PAN identifier.
    #[must_use]
    pub const fn extended_pan_id(&self) -> u64 {
        self.extended_pan_id
    }

    /// Return whether the network is allowing MAC associations.
    #[must_use]
    pub const fn allowing_join(&self) -> bool {
        self.allowing_join
    }

    /// Return the Stack Profile associated with the network.
    #[must_use]
    pub const fn stack_profile(&self) -> u8 {
        self.stack_profile
    }

    /// Return the instance of the Network.
    #[must_use]
    pub const fn nwk_update_id(&self) -> u8 {
        self.nwk_update_id
    }
}
