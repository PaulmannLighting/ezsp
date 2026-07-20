use tokio::sync::oneshot::Sender;

use crate::frame::parameters::messaging::{send_broadcast, send_multicast, send_unicast};
use crate::{Error, Parameters};

/// Messages processed by the actor-based NCP.
#[derive(Debug)]
pub enum Message {
    /// An incoming EZSP response or asynchronous callback.
    Received(Parameters),

    /// A unicast command and its response channel.
    Unicast {
        /// The typed EZSP command.
        command: Box<send_unicast::Command>,
        /// Returns the APS sequence or transmission error.
        response: Sender<Result<u8, Error>>,
    },

    /// A multicast command and its response channel.
    Multicast {
        /// The typed EZSP command.
        command: Box<send_multicast::Command>,
        /// Returns the APS sequence or transmission error.
        response: Sender<Result<u8, Error>>,
    },

    /// A broadcast command and its response channel.
    Broadcast {
        /// The typed EZSP command.
        command: Box<send_broadcast::Command>,
        /// Returns the APS sequence or transmission error.
        response: Sender<Result<u8, Error>>,
    },
}
