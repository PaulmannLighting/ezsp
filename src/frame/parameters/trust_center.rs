//! Trust Center Frames

pub(crate) mod aes_mmo_hash;
pub(crate) mod broadcast_network_key_switch;
pub(crate) mod broadcast_next_network_key;
pub mod handler;
pub(crate) mod remove_device;
pub(crate) mod unicast_nwk_key_update;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    AesMmoHash(aes_mmo_hash::Command),
    BroadcastNetworkKeySwitch(broadcast_network_key_switch::Command),
    BroadcastNextNetworkKey(broadcast_next_network_key::Command),
    RemoveDevice(remove_device::Command),
    UnicastNwkKeyUpdate(unicast_nwk_key_update::Command),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    AesMmoHash(aes_mmo_hash::Response),
    BroadcastNetworkKeySwitch(broadcast_network_key_switch::Response),
    BroadcastNextNetworkKey(broadcast_next_network_key::Response),
    RemoveDevice(remove_device::Response),
    UnicastNwkKeyUpdate(unicast_nwk_key_update::Response),
    Handler(handler::Handler),
}
