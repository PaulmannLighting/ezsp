pub mod switch_network_key;
pub mod zigbee_key_establishment;

/// The handler for the security command.
#[allow(variant_size_differences, clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// The handler for the switch network key command.
    SwitchNetworkKey(switch_network_key::Handler),
    /// The handler for the Zigbee key establishment command.
    ZigbeeKeyEstablishment(zigbee_key_establishment::Handler),
}

impl From<switch_network_key::Handler> for Handler {
    fn from(handler: switch_network_key::Handler) -> Self {
        Self::SwitchNetworkKey(handler)
    }
}

impl From<zigbee_key_establishment::Handler> for Handler {
    fn from(handler: zigbee_key_establishment::Handler) -> Self {
        Self::ZigbeeKeyEstablishment(handler)
    }
}
