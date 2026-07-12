use apis_saltans_hw::nwk::Metadata;

use crate::parameters::messaging::handler::IncomingMessage;

impl From<&IncomingMessage> for Metadata {
    fn from(incoming_message: &IncomingMessage) -> Self {
        Self::new(
            Some(incoming_message.last_hop_lqi()),
            Some(incoming_message.last_hop_rssi().into()),
            Some(incoming_message.binding_index().into()),
            incoming_message.source_route_overhead(),
        )
    }
}
