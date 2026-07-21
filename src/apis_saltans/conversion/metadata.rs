//! Reception metadata conversion for incoming APS messages.
//!
//! EZSP link quality, RSSI, binding index, and source-route overhead are
//! represented in the NWK envelope metadata.

use apis_saltans_hw::nwk::Metadata;

use crate::DefragmentedMessage;

impl From<&DefragmentedMessage> for Metadata {
    fn from(message: &DefragmentedMessage) -> Self {
        Self::new(
            Some(message.last_hop_lqi()),
            Some(message.last_hop_rssi().into()),
            Some(message.binding_index().into()),
            message.source_route_overhead(),
        )
    }
}
