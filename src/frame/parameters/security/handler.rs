//! Security command handlers.

pub use self::switch_network_key::Handler as SwitchNetworkKey;
pub use self::zigbee_key_establishment::Handler as ZigbeeKeyEstablishment;

mod switch_network_key;
mod zigbee_key_establishment;

/// The handler for the security command.
#[allow(variant_size_differences)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// The handler for the switch network key command.
    SwitchNetworkKey(SwitchNetworkKey),
    /// The handler for the Zigbee key establishment command.
    ZigbeeKeyEstablishment(Box<ZigbeeKeyEstablishment>),
}
