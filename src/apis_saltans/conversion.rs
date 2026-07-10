//! Conversion implementations between EZSP callback data and `apis-saltans` types.
//!
//! These conversions are used by the feature-gated NCP driver adapter and event
//! handler. They translate incoming APS frames, endpoint descriptors, scan
//! results, join/leave callbacks, and stack status callbacks into the types
//! expected by `apis_saltans_aps`, `apis_saltans_core`,
//! `apis_saltans_hw`, and the EZSP endpoint registration command.

use apis_saltans_aps::data::Header;
use apis_saltans_aps::{Data, Destination, Extended};
use apis_saltans_core::endpoint::{Application, Broadcast};
use apis_saltans_core::{Endpoint, GroupId};
use bytes::Bytes;
use log::trace;

pub use self::error::ParseApsFrameError;
use crate::ember::message::Incoming;
use crate::parameters::messaging::handler::IncomingMessage;

mod address;
mod aps_header;
mod clusters;
mod envelope;
mod error;
mod event;
mod found_network;
mod metadata;
mod scanned_channel;

impl TryFrom<IncomingMessage> for Data<Bytes> {
    type Error = ParseApsFrameError;

    fn try_from(message: IncomingMessage) -> Result<Self, Self::Error> {
        let aps_frame = message.aps_frame();
        let typ = message.typ().map_err(ParseApsFrameError::MessageType)?;

        let destination_endpoint = aps_frame.destination_endpoint();
        let destination = match typ {
            Incoming::Broadcast | Incoming::BroadcastLoopback => Destination::Broadcast(
                Broadcast::try_from(destination_endpoint)
                    .map_err(ParseApsFrameError::BroadcastEndpoint)?,
            ),
            Incoming::Unicast | Incoming::UnicastReply => Destination::Unicast(
                Application::try_from(destination_endpoint)
                    .map_err(ParseApsFrameError::ApplicationEndpoint)?,
            ),
            Incoming::Multicast | Incoming::MulticastLoopback => Destination::Group(
                GroupId::try_from(aps_frame.group_id()).map_err(ParseApsFrameError::GroupId)?,
            ),
            Incoming::ManyToOneRouteRequest => unreachable!("EZSP does not allow this."),
        };

        let source_endpoint = Endpoint::try_from(aps_frame.source_endpoint())
            .map_err(|endpoint| ParseApsFrameError::SourceEndpoint(endpoint.into()))?;

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

        let header = Header::new(
            destination,
            aps_frame.cluster_id(),
            aps_frame.profile_id(),
            source_endpoint,
            aps_frame.sequence(),
            extended,
        );
        Ok(Self::raw(
            header,
            message.into_message().into_iter().collect(),
        ))
    }
}
