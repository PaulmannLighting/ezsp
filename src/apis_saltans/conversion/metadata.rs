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
