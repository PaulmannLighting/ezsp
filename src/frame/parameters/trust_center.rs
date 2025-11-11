//! Trust Center Frames

pub mod aes_mmo_hash;
pub mod broadcast_network_key_switch;
pub mod broadcast_next_network_key;
pub mod handler;
pub mod remove_device;
pub mod unicast_nwk_key_update;

/// Trust Center response parameters.
#[expect(variant_size_differences)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Response to the `aes_mmo_hash` command.
    AesMmoHash(Box<aes_mmo_hash::Response>),
    /// Response to the `broadcast_network_key_switch` command.
    BroadcastNetworkKeySwitch(broadcast_network_key_switch::Response),
    /// Response to the `broadcast_next_network_key` command.
    BroadcastNextNetworkKey(broadcast_next_network_key::Response),
    /// Response to the `remove_device` command.
    RemoveDevice(remove_device::Response),
    /// Response to the `unicast_nwk_key_update` command.
    UnicastNwkKeyUpdate(unicast_nwk_key_update::Response),
}
