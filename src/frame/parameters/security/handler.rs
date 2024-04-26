pub mod switch_network_key;
pub mod zigbee_key_establishment;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    SwitchNetworkKey(switch_network_key::Handler),
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
