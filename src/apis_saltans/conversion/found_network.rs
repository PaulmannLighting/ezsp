//! Active-scan result conversion.
//!
//! Channel, PAN identifiers, join permission, stack profile, update ID, link
//! quality, and RSSI are preserved. Results outside the Zigbee page-zero
//! channel range are rejected.

use apis_saltans_hw::{Channel, FoundNetwork, NetworkDescriptor};

use crate::parameters::networking::handler::NetworkFound;

impl TryFrom<NetworkFound> for FoundNetwork {
    type Error = u8;

    fn try_from(network_found: NetworkFound) -> Result<Self, Self::Error> {
        let network = network_found.network_found();
        let channel = network.channel();
        let channel = Channel::new(channel).ok_or(channel)?;

        Ok(Self::new(
            NetworkDescriptor::new(
                channel,
                network.pan_id(),
                network.extended_pan_id().into(),
                network.allowing_join(),
                network.stack_profile(),
                network.nwk_update_id(),
            ),
            network_found.last_hop_lqi(),
            network_found.last_hop_rssi(),
        ))
    }
}
