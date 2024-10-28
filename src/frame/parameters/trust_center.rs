//! Trust Center Frames

pub mod aes_mmo_hash;
pub mod broadcast_network_key_switch;
pub mod broadcast_next_network_key;
pub mod handler;
pub mod remove_device;
pub mod unicast_nwk_key_update;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    AesMmoHash(aes_mmo_hash::Response),
    BroadcastNetworkKeySwitch(broadcast_network_key_switch::Response),
    BroadcastNextNetworkKey(broadcast_next_network_key::Response),
    RemoveDevice(remove_device::Response),
    UnicastNwkKeyUpdate(unicast_nwk_key_update::Response),
}
