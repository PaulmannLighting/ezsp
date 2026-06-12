//! Conversion implementations from EZSP data structures to Zigbee Nwk data structures.

use aps::Destination;
use zigbee::Profile;
use zigbee_hw::Command;

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
mod zcl_frame;
mod zdp_frame;

impl TryFrom<DefragmentedMessage> for aps::Data<Command> {
    type Error = ParseApsFrameError;

    fn try_from(message: DefragmentedMessage) -> Result<Self, Self::Error> {
        let typ = match message.typ() {
            Ok(typ) => typ,
            Err(id) => return Err(ParseApsFrameError::InvalidMessageType(id)),
        };

        let aps_frame = message.aps_frame();
        let profile = Profile::try_from(aps_frame.profile_id())
            .map_err(ParseApsFrameError::InvalidProfile)?;

        Ok(Self::new(
            match typ {
                Incoming::Broadcast | Incoming::BroadcastLoopback => {
                    Destination::Broadcast(aps_frame.destination_endpoint())
                }
                Incoming::Unicast | Incoming::UnicastReply => {
                    Destination::Unicast(aps_frame.destination_endpoint())
                }
                Incoming::Multicast | Incoming::MulticastLoopback => {
                    Destination::Group(aps_frame.group_id())
                }
                Incoming::ManyToOneRouteRequest => unreachable!("EZSP does not allow this."),
            },
            aps_frame.cluster_id(),
            aps_frame.profile_id(),
            aps_frame.source_endpoint(),
            aps_frame.sequence(),
            None,
            match profile {
                Profile::Network => zdp::Frame::<zdp::Command>::try_from(message)
                    .map(Command::Zdp)
                    .map_err(ParseApsFrameError::ParseZdpFrameError),
                Profile::ZigbeeHomeAutomation
                | Profile::SmartEnergy
                | Profile::TouchLink
                | Profile::BuildingAutomation
                | Profile::HealthCare
                | Profile::RemoteControl => zcl::Frame::<zcl::Cluster>::try_from(message)
                    .map(Command::Zcl)
                    .map_err(ParseApsFrameError::ParseZclFrameError),
            }?,
        ))
    }
}
