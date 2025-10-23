//! Message handlers.

pub use self::id_conflict::Handler as IdConflict;
pub use self::incoming_many_to_one_route_request::Handler as IncomingManyToOneRouteRequest;
pub use self::incoming_message::Handler as IncomingMessage;
pub use self::incoming_network_status::Handler as IncomingNetworkStatus;
pub use self::incoming_route_error::Handler as IncomingRouteError;
pub use self::incoming_route_record::Handler as IncomingRouteRecord;
pub use self::incoming_sender_eui64::Handler as IncomingSenderEui64;
pub use self::mac_filter_match_message::Handler as MacFilterMatchMessage;
pub use self::mac_passthrough_message::Handler as MacPassthroughMessage;
pub use self::message_sent::Handler as MessageSent;
pub use self::poll::Handler as Poll;
pub use self::poll_complete::Handler as PollComplete;
pub use self::raw_transmit_complete::Handler as RawTransmitComplete;

mod id_conflict;
mod incoming_many_to_one_route_request;
mod incoming_message;
mod incoming_network_status;
mod incoming_route_error;
mod incoming_route_record;
mod incoming_sender_eui64;
mod mac_filter_match_message;
mod mac_passthrough_message;
mod message_sent;
mod poll;
mod poll_complete;
mod raw_transmit_complete;

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
    /// An incoming route record handler.
    IncomingRouteRecord(IncomingRouteRecord),
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
