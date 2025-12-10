use zcl::{Cluster, Frame, ParseFrameError};

use crate::parameters::messaging::handler::IncomingMessage;

impl TryFrom<IncomingMessage> for Frame<Cluster> {
    type Error = ParseFrameError;

    fn try_from(frame: IncomingMessage) -> Result<Self, Self::Error> {
        Self::parse(
            frame.aps_frame().cluster_id(),
            frame.into_message().drain(..),
        )
    }
}
