//! Conversion of complete incoming EZSP messages to APSDE indications.
//!
//! The indication preserves the sender, endpoints, multicast group, profile,
//! cluster, payload, and link quality exposed by EZSP. EZSP does not attach a
//! reception timestamp or APS key-pair handle, so both backend-defined context
//! values are `()`.

use apis_saltans_hw::aps::apsde::{
    DataIndication, IndicationMetadata, IndicationStatus, IndividualEndpoint, NetworkAddress,
    ReceivedDestination, Security, Source,
};
use apis_saltans_hw::core::{Endpoint, GroupId};
use bytes::Bytes;

use crate::DefragmentedMessage;
use crate::apis_saltans::conversion::ParseApsFrameError;
use crate::ember::message::Incoming;

const COORDINATOR_NETWORK_ADDRESS: u16 = 0x0000;

impl TryFrom<DefragmentedMessage> for DataIndication<Bytes, ()> {
    type Error = ParseApsFrameError;

    fn try_from(message: DefragmentedMessage) -> Result<Self, Self::Error> {
        let aps_frame = message.aps_frame();
        let typ = message.typ().map_err(ParseApsFrameError::MessageType)?;
        let destination_endpoint = individual_endpoint(
            aps_frame.destination_endpoint(),
            ParseApsFrameError::DestinationEndpoint,
        )?;
        let destination = match typ {
            Incoming::Broadcast
            | Incoming::BroadcastLoopback
            | Incoming::Unicast
            | Incoming::UnicastReply => ReceivedDestination::Network {
                address: NetworkAddress::new(COORDINATOR_NETWORK_ADDRESS)
                    .expect("the coordinator address is a valid APSDE network address"),
                endpoint: destination_endpoint,
            },
            Incoming::Multicast | Incoming::MulticastLoopback => ReceivedDestination::Group(
                GroupId::new(aps_frame.group_id())
                    .ok_or_else(|| ParseApsFrameError::GroupId(aps_frame.group_id()))?,
            ),
            Incoming::ManyToOneRouteRequest => {
                return Err(ParseApsFrameError::MessageType(typ.into()));
            }
        };
        let source_endpoint = individual_endpoint(
            aps_frame.source_endpoint(),
            ParseApsFrameError::SourceEndpoint,
        )?;
        let source_address = NetworkAddress::new(message.sender())
            .ok_or_else(|| ParseApsFrameError::SourceAddress(message.sender()))?;
        let metadata = IndicationMetadata::new(
            destination,
            Source::Network {
                address: source_address,
                endpoint: source_endpoint,
            },
            aps_frame.profile_id(),
            aps_frame.cluster_id(),
            IndicationStatus::success(),
            Security::NetworkKey,
            message.last_hop_lqi(),
            (),
        );

        Ok(Self::new(
            metadata,
            message.into_message().into_iter().collect(),
        ))
    }
}

fn individual_endpoint(
    endpoint: u8,
    error: fn(u8) -> ParseApsFrameError,
) -> Result<IndividualEndpoint, ParseApsFrameError> {
    IndividualEndpoint::new(Endpoint::from(endpoint)).ok_or_else(|| error(endpoint))
}
