use tokio::sync::oneshot::Sender;

use crate::Callback;
use crate::ember::Status;
use crate::ember::aps::Frame;
use crate::parameters::networking::handler::{EnergyScanResult, NetworkFound};
use crate::types::ByteSizedVec;

pub enum ToTransmitter {
    Unicast
    SendReply {
        node_id: u16,
        aps_frame: Frame,
        payload: ByteSizedVec<u8>,
    },
}

/// Messages exchanged with the NCP event handler.
///
/// The event handler receives raw EZSP callbacks, one-shot registration
/// requests for scans and outgoing message confirmations, and a termination
/// signal used by [`Ncp::terminate`](crate::Ncp::terminate).
#[derive(Debug)]
pub enum ToReceiver {
    /// An incoming callback.
    Callback(Box<Callback>),

    /// Registers a receiver for the next active network scan.
    NetworkScan(Sender<Vec<NetworkFound>>),

    /// Registers a receiver for the next energy scan.
    ChannelScan(Sender<Vec<EnergyScanResult>>),

    /// Registers a receiver for a `messageSent` callback with the given tag.
    Sent {
        /// The message tag.
        tag: u8,
        /// The result sender for the stack status reported by `messageSent`.
        sender: Sender<Result<Status, u8>>,
    },

    /// Stops the event handler.
    Terminate,
}

impl From<Box<Callback>> for ToReceiver {
    fn from(callback: Box<Callback>) -> Self {
        Self::Callback(callback)
    }
}

impl From<Callback> for ToReceiver {
    fn from(callback: Callback) -> Self {
        Self::from(Box::new(callback))
    }
}

impl From<Sender<Vec<NetworkFound>>> for ToReceiver {
    fn from(sender: Sender<Vec<NetworkFound>>) -> Self {
        Self::NetworkScan(sender)
    }
}

impl From<Sender<Vec<EnergyScanResult>>> for ToReceiver {
    fn from(sender: Sender<Vec<EnergyScanResult>>) -> Self {
        Self::ChannelScan(sender)
    }
}
