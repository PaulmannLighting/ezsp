use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::Sender;

use crate::ember::Status;
use crate::ember::aps::Frame;
use crate::ncp::transmitter::MulticastOptions;
use crate::parameters::networking::handler::{EnergyScanResult, NetworkFound};
use crate::types::ByteSizedVec;
use crate::{Callback, Error};

/// Requests handled by the NCP transmitter actor.
pub enum ToTransmitter {
    /// Finds the source endpoint for an outgoing cluster.
    SourceEndpoint {
        /// The application profile identifier.
        profile_id: u16,
        /// The outgoing cluster identifier.
        cluster_id: u16,
        /// The channel used to return the matching endpoint or an error.
        response: Sender<Result<u8, Error>>,
    },

    /// Stops the transmitter and its background event handler.
    Terminate {
        /// The channel used to return the termination result.
        response: Sender<Result<(), SendError<ToReceiver>>>,
    },

    /// Sends a unicast APS message.
    Unicast {
        /// The destination node's short identifier.
        short_id: u16,
        /// The application profile identifier.
        profile_id: u16,
        /// The cluster identifier.
        cluster_id: u16,
        /// The destination endpoint.
        destination_endpoint: u8,
        /// The application payload.
        payload: Box<[u8]>,
        /// The channel used to return the transmission result.
        response: Sender<Result<(), Error>>,
    },

    /// Sends a multicast APS message.
    Multicast {
        /// The destination group identifier.
        group_id: u16,
        /// The multicast delivery options.
        options: MulticastOptions,
        /// The application profile identifier.
        profile_id: u16,
        /// The cluster identifier.
        cluster_id: u16,
        /// The destination endpoint.
        destination_endpoint: u8,
        /// The application payload.
        payload: Box<[u8]>,
        /// The channel used to return the transmission result.
        response: Sender<Result<(), Error>>,
    },

    /// Sends a broadcast APS message.
    Broadcast {
        /// The broadcast destination's short identifier.
        short_id: u16,
        /// The maximum broadcast radius.
        radius: u8,
        /// The application profile identifier.
        profile_id: u16,
        /// The cluster identifier.
        cluster_id: u16,
        /// The destination endpoint.
        destination_endpoint: u8,
        /// The application payload.
        payload: Box<[u8]>,
        /// The channel used to return the transmission result.
        response: Sender<Result<(), Error>>,
    },

    /// Starts an active network scan.
    ScanNetworks {
        /// The bit mask selecting the channels to scan.
        channel_mask: u32,
        /// The scan duration exponent.
        duration: u8,
        /// The channel used to return discovered networks or an error.
        response: Sender<Result<Vec<NetworkFound>, Error>>,
    },

    /// Starts an energy scan.
    ScanChannels {
        /// The bit mask selecting the channels to scan.
        channel_mask: u32,
        /// The scan duration exponent.
        duration: u8,
        /// The channel used to return energy scan results or an error.
        response: Sender<Result<Vec<EnergyScanResult>, Error>>,
    },

    /// Sends an APS fragmentation reply generated for an incoming message.
    SendReply {
        /// The node that sent the fragmented message.
        node_id: u16,
        /// The APS frame from the incoming message.
        aps_frame: Frame,
        /// The fragmentation reply payload.
        payload: Box<ByteSizedVec<u8>>,
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
