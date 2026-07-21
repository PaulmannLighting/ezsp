//! Conversion of complete incoming EZSP APS messages.
//!
//! Broadcast and unicast messages preserve the destination endpoint;
//! multicasts use the APS group ID. The conversion also preserves profile,
//! cluster, source endpoint, APS sequence, and payload bytes. Reserved endpoint
//! or group identifiers are reported as [`ParseApsFrameError`].

use apis_saltans_hw::aps::data::Header;
use apis_saltans_hw::aps::{Data, Destination};
use apis_saltans_hw::core::{Endpoint, GroupId};
use bytes::Bytes;

use crate::DefragmentedMessage;
use crate::apis_saltans::conversion::ParseApsFrameError;
use crate::ember::message::Incoming;

impl TryFrom<DefragmentedMessage> for Data<Bytes> {
    type Error = ParseApsFrameError;

    fn try_from(message: DefragmentedMessage) -> Result<Self, Self::Error> {
        let aps_frame = message.aps_frame();
        let typ = message.typ().map_err(ParseApsFrameError::MessageType)?;

        let destination_endpoint = aps_frame.destination_endpoint();
        let destination = match typ {
            Incoming::Broadcast | Incoming::BroadcastLoopback => Destination::Broadcast(
                Endpoint::try_from(destination_endpoint)
                    .map_err(ParseApsFrameError::InvalidEndpoint)?,
            ),
            Incoming::Unicast | Incoming::UnicastReply => Destination::Unicast(
                Endpoint::try_from(destination_endpoint)
                    .map_err(ParseApsFrameError::InvalidEndpoint)?,
            ),
            Incoming::Multicast | Incoming::MulticastLoopback => Destination::Group(
                GroupId::try_from(aps_frame.group_id()).map_err(ParseApsFrameError::GroupId)?,
            ),
            Incoming::ManyToOneRouteRequest => unreachable!("EZSP does not allow this."),
        };

        let source_endpoint = Endpoint::try_from(aps_frame.source_endpoint())
            .map_err(|endpoint| ParseApsFrameError::SourceEndpoint(endpoint.into()))?;

        let header = Header::new(
            destination,
            aps_frame.cluster_id(),
            aps_frame.profile_id(),
            source_endpoint,
            aps_frame.sequence(),
            None,
        );
        Ok(Self::raw(
            header,
            message.into_message().into_iter().collect(),
        ))
    }
}
