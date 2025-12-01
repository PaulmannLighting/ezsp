use std::collections::BTreeMap;

use macaddr::MacAddr8;

use crate::ember::concentrator;
use crate::ezsp::{config, decision, policy};

/// Configuration for a Zigbee device.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceConfig {
    pub(crate) concentrator: concentrator::Parameters,
    pub(crate) configuration: BTreeMap<config::Id, u16>,
    pub(crate) policy: BTreeMap<policy::Id, decision::Id>,
    pub(crate) link_key: [u8; 16],
    pub(crate) extended_pan_id: MacAddr8,
    pub(crate) pan_id: u16,
    pub(crate) radio_channel: u8,
}

impl DeviceConfig {
    /// Create a new device configuration.
    #[must_use]
    pub const fn new(
        concentrator: concentrator::Parameters,
        configuration: BTreeMap<config::Id, u16>,
        policy: BTreeMap<policy::Id, decision::Id>,
        link_key: [u8; 16],
        extended_pan_id: MacAddr8,
        pan_id: u16,
        radio_channel: u8,
    ) -> Self {
        Self {
            concentrator,
            configuration,
            policy,
            link_key,
            extended_pan_id,
            pan_id,
            radio_channel,
        }
    }
}
