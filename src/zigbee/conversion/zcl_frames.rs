use zcl::{Cluster, Frame, ParseFrameError};

use crate::defragmentation::DefragmentedMessage;

impl TryFrom<DefragmentedMessage> for Frame<Cluster> {
    type Error = ParseFrameError;

    fn try_from(frame: DefragmentedMessage) -> Result<Self, Self::Error> {
        Self::parse(
            frame.aps_frame().cluster_id(),
            frame.into_message().drain(..),
        )
    }
}
