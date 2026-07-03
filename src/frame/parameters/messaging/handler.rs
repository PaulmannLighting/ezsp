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
crate::frame::parameters::parameter_enum!(
    Handler,
    IdConflict,
    IncomingManyToOneRouteRequest,
    IncomingMessage,
    IncomingNetworkStatus,
    IncomingRouteError,
    IncomingRouteRecord,
    IncomingSenderEui64,
    MacFilterMatchMessage,
    MacPassthroughMessage,
    MessageSent,
    Poll,
    PollComplete,
    RawTransmitComplete
);
