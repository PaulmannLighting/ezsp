//! Conversion implementations from EZSP data structures to Zigbee Nwk data structures.

use aps::Destination;

pub use self::error::ParseApsFrameError;
use crate::DefragmentedMessage;
use crate::ember::message::Incoming;

mod address;
mod child_join;
mod error;
mod found_network;
mod scanned_channel;
mod status;
mod trust_center_join;

impl TryFrom<DefragmentedMessage> for aps::Data<Vec<u8>> {
    type Error = ParseApsFrameError;

    fn try_from(message: DefragmentedMessage) -> Result<Self, Self::Error> {
        let typ = match message.typ() {
            Ok(typ) => typ,
            Err(id) => return Err(ParseApsFrameError::InvalidMessageType(id)),
        };

        let aps_frame = message.aps_frame();

        Ok(Self::new(
            match typ {
                Incoming::Broadcast | Incoming::BroadcastLoopback => {
                    Destination::Broadcast(aps_frame.destination_endpoint().into())
                }
                Incoming::Unicast | Incoming::UnicastReply => {
                    Destination::Unicast(aps_frame.destination_endpoint().into())
                }
                Incoming::Multicast | Incoming::MulticastLoopback => {
                    Destination::Group(aps_frame.group_id())
                }
                Incoming::ManyToOneRouteRequest => unreachable!("EZSP does not allow this."),
            },
            aps_frame.cluster_id(),
            aps_frame.profile_id(),
            aps_frame.source_endpoint().into(),
            aps_frame.sequence(),
            None,
            message.into_message(),
        ))
    }
}
