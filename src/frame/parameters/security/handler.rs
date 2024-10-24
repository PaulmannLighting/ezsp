//! Security command handlers.

mod switch_network_key;
mod zigbee_key_establishment;

pub use switch_network_key::Handler as SwitchNetworkKey;
pub use zigbee_key_establishment::Handler as ZigbeeKeyEstablishment;

/// The handler for the security command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// The handler for the switch network key command.
    SwitchNetworkKey(SwitchNetworkKey),
    /// The handler for the Zigbee key establishment command.
    ZigbeeKeyEstablishment(ZigbeeKeyEstablishment),
}

impl From<SwitchNetworkKey> for Handler {
    fn from(handler: SwitchNetworkKey) -> Self {
        Self::SwitchNetworkKey(handler)
    }
}

impl From<ZigbeeKeyEstablishment> for Handler {
    fn from(handler: ZigbeeKeyEstablishment) -> Self {
        Self::ZigbeeKeyEstablishment(handler)
    }
}
