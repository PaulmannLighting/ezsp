pub mod id_conflict;
pub mod incoming_many_to_one_route_request;
pub mod incoming_message;
pub mod incoming_network_status;
pub mod incoming_route_error;
pub mod incoming_sender_eui64;
pub mod mac_filter_match_message;
pub mod mac_passthrough_message;
pub mod message_sent;
pub mod poll;
pub mod poll_complete;
pub mod raw_transmit_complete_handler;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    IdConflict(id_conflict::Handler),
    IncomingManyToOneRouteRequest(incoming_many_to_one_route_request::Handler),
    IncomingMessage(incoming_message::Handler),
    IncomingNetworkStatus(incoming_network_status::Handler),
    IncomingRouteError(incoming_route_error::Handler),
    IncomingSenderEui64(incoming_sender_eui64::Handler),
    MacFilterMatchMessage(mac_filter_match_message::Handler),
    MacPassthroughMessage(mac_passthrough_message::Handler),
    MessageSent(message_sent::Handler),
    Poll(poll::Handler),
    PollComplete(poll_complete::Handler),
    RawTransmitCompleteHandler(raw_transmit_complete_handler::Handler),
}

impl From<id_conflict::Handler> for Handler {
    fn from(handler: id_conflict::Handler) -> Self {
        Self::IdConflict(handler)
    }
}

impl From<incoming_many_to_one_route_request::Handler> for Handler {
    fn from(handler: incoming_many_to_one_route_request::Handler) -> Self {
        Self::IncomingManyToOneRouteRequest(handler)
    }
}

impl From<incoming_message::Handler> for Handler {
    fn from(handler: incoming_message::Handler) -> Self {
        Self::IncomingMessage(handler)
    }
}

impl From<incoming_network_status::Handler> for Handler {
    fn from(handler: incoming_network_status::Handler) -> Self {
        Self::IncomingNetworkStatus(handler)
    }
}

impl From<incoming_route_error::Handler> for Handler {
    fn from(handler: incoming_route_error::Handler) -> Self {
        Self::IncomingRouteError(handler)
    }
}

impl From<incoming_sender_eui64::Handler> for Handler {
    fn from(handler: incoming_sender_eui64::Handler) -> Self {
        Self::IncomingSenderEui64(handler)
    }
}

impl From<mac_filter_match_message::Handler> for Handler {
    fn from(handler: mac_filter_match_message::Handler) -> Self {
        Self::MacFilterMatchMessage(handler)
    }
}

impl From<mac_passthrough_message::Handler> for Handler {
    fn from(handler: mac_passthrough_message::Handler) -> Self {
        Self::MacPassthroughMessage(handler)
    }
}

impl From<message_sent::Handler> for Handler {
    fn from(handler: message_sent::Handler) -> Self {
        Self::MessageSent(handler)
    }
}

impl From<poll::Handler> for Handler {
    fn from(handler: poll::Handler) -> Self {
        Self::Poll(handler)
    }
}

impl From<poll_complete::Handler> for Handler {
    fn from(handler: poll_complete::Handler) -> Self {
        Self::PollComplete(handler)
    }
}

impl From<raw_transmit_complete_handler::Handler> for Handler {
    fn from(handler: raw_transmit_complete_handler::Handler) -> Self {
        Self::RawTransmitCompleteHandler(handler)
    }
}
