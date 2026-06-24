//! Conversion implementations from EZSP data structures to Zigbee Nwk data structures.

use aps::{Destination, Extended};
use log::trace;

pub use self::error::ParseApsFrameError;
use crate::ember::message::Incoming;
use crate::parameters::messaging::handler::IncomingMessage;

mod address;
mod child_join;
mod error;
mod found_network;
mod scanned_channel;
mod status;
mod trust_center_join;

impl TryFrom<IncomingMessage> for aps::Data<Vec<u8>> {
    type Error = ParseApsFrameError;

    fn try_from(message: IncomingMessage) -> Result<Self, Self::Error> {
        let aps_frame = message.aps_frame();
        let typ = message.typ().map_err(ParseApsFrameError::MessageType)?;

        let destination = match typ {
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
        };

        let extended = match aps_frame.fragmentation() {
            Some((0, Some(size))) => {
                trace!("Received initial fragment.");
                Some(Extended::first_fragment(size))
            }
            Some((index, None)) => {
                trace!("Received follow-up fragment.");
                Some(Extended::followup_fragment(index))
            }
            None => {
                trace!("Received non-fragmented frame.");
                None
            }
            Some((index, Some(size))) => {
                trace!("Received invalid fragmentation information: {index}/{size}");
                return Err(ParseApsFrameError::Fragmentation { index, size });
            }
        };

        Ok(Self::new(
            destination,
            aps_frame.cluster_id(),
            aps_frame.profile_id(),
            aps_frame.source_endpoint().into(),
            aps_frame.sequence(),
            extended,
            message.into_message().into_iter().collect(),
        ))
    }
}
