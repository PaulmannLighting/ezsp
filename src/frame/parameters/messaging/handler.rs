//! Message handlers.

mod id_conflict;
mod incoming_many_to_one_route_request;
mod incoming_message;
mod incoming_network_status;
mod incoming_route_error;
mod incoming_sender_eui64;
mod mac_filter_match_message;
mod mac_passthrough_message;
mod message_sent;
mod poll;
mod poll_complete;
mod raw_transmit_complete;

pub use id_conflict::Handler as IdConflict;
pub use incoming_many_to_one_route_request::Handler as IncomingManyToOneRouteRequest;
pub use incoming_message::Handler as IncomingMessage;
pub use incoming_network_status::Handler as IncomingNetworkStatus;
pub use incoming_route_error::Handler as IncomingRouteError;
pub use incoming_sender_eui64::Handler as IncomingSenderEui64;
pub use mac_filter_match_message::Handler as MacFilterMatchMessage;
pub use mac_passthrough_message::Handler as MacPassthroughMessage;
pub use message_sent::Handler as MessageSent;
pub use poll::Handler as Poll;
pub use poll_complete::Handler as PollComplete;
pub use raw_transmit_complete::Handler as RawTransmitComplete;

/// Message handlers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// An ID conflict handler.
    IdConflict(IdConflict),
    /// An incoming many-to-one route request handler.
    IncomingManyToOneRouteRequest(IncomingManyToOneRouteRequest),
    /// An incoming message handler.
    IncomingMessage(IncomingMessage),
    /// An incoming network status handler.
    IncomingNetworkStatus(IncomingNetworkStatus),
    /// An incoming route error handler.
    IncomingRouteError(IncomingRouteError),
    /// An incoming sender EUI64 handler.
    IncomingSenderEui64(IncomingSenderEui64),
    /// A MAC filter match message handler.
    MacFilterMatchMessage(MacFilterMatchMessage),
    /// A MAC passthrough message handler.
    MacPassthroughMessage(MacPassthroughMessage),
    /// A message sent handler.
    MessageSent(MessageSent),
    /// A poll handler.
    Poll(Poll),
    /// A poll complete handler.
    PollComplete(PollComplete),
    /// A raw transmit complete handler.
    RawTransmitComplete(RawTransmitComplete),
}

impl From<IdConflict> for Handler {
    fn from(handler: IdConflict) -> Self {
        Self::IdConflict(handler)
    }
}

impl From<IncomingManyToOneRouteRequest> for Handler {
    fn from(handler: IncomingManyToOneRouteRequest) -> Self {
        Self::IncomingManyToOneRouteRequest(handler)
    }
}

impl From<IncomingMessage> for Handler {
    fn from(handler: IncomingMessage) -> Self {
        Self::IncomingMessage(handler)
    }
}

impl From<IncomingNetworkStatus> for Handler {
    fn from(handler: IncomingNetworkStatus) -> Self {
        Self::IncomingNetworkStatus(handler)
    }
}

impl From<IncomingRouteError> for Handler {
    fn from(handler: IncomingRouteError) -> Self {
        Self::IncomingRouteError(handler)
    }
}

impl From<IncomingSenderEui64> for Handler {
    fn from(handler: IncomingSenderEui64) -> Self {
        Self::IncomingSenderEui64(handler)
    }
}

impl From<MacFilterMatchMessage> for Handler {
    fn from(handler: MacFilterMatchMessage) -> Self {
        Self::MacFilterMatchMessage(handler)
    }
}

impl From<MacPassthroughMessage> for Handler {
    fn from(handler: MacPassthroughMessage) -> Self {
        Self::MacPassthroughMessage(handler)
    }
}

impl From<MessageSent> for Handler {
    fn from(handler: MessageSent) -> Self {
        Self::MessageSent(handler)
    }
}

impl From<Poll> for Handler {
    fn from(handler: Poll) -> Self {
        Self::Poll(handler)
    }
}

impl From<PollComplete> for Handler {
    fn from(handler: PollComplete) -> Self {
        Self::PollComplete(handler)
    }
}

impl From<RawTransmitComplete> for Handler {
    fn from(handler: RawTransmitComplete) -> Self {
        Self::RawTransmitComplete(handler)
    }
}
