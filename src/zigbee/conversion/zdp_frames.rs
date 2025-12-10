use zdp::{Command, Frame};

pub use super::zdp_frame_from_incoming_message_error::ZdpFrameFromIncomingMessageError;
use crate::parameters::messaging::handler::IncomingMessage;

impl TryFrom<IncomingMessage> for Frame<Command> {
    type Error = ZdpFrameFromIncomingMessageError;

    fn try_from(frame: IncomingMessage) -> Result<Self, Self::Error> {
        let aps_frame = frame.aps_frame();

        if aps_frame.source_endpoint() != 0 {
            return Err(Self::Error::InvalidSourceEndpoint(
                aps_frame.source_endpoint(),
            ));
        } else if aps_frame.destination_endpoint() != 0 {
            return Err(Self::Error::InvalidDestinationEndpoint(
                aps_frame.source_endpoint(),
            ));
        }

        Self::parse(aps_frame.cluster_id(), frame.into_message().drain(..))
            .map_err(Self::Error::ParseFrameError)
    }
}
