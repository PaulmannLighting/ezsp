//! Complete incoming APS messages with unrestricted payloads.

use crate::ember::NodeId;
use crate::ember::aps::Frame as ApsFrame;
use crate::ember::message::Incoming;
use crate::parameters::messaging::handler::IncomingMessage;

/// A complete incoming APS message with an owned, unrestricted payload.
#[derive(Debug)]
pub struct DefragmentedMessage {
    typ: u8,
    aps_frame: ApsFrame,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
    sender: NodeId,
    binding_index: u8,
    address_index: u8,
    payload: Box<[u8]>,
    source_route_overhead: Option<u8>,
}

impl DefragmentedMessage {
    /// Returns the incoming message type.
    ///
    /// # Errors
    ///
    /// Returns the raw message type if it is not a recognized [`Incoming`] value.
    pub fn typ(&self) -> Result<Incoming, u8> {
        Incoming::try_from(self.typ).map_err(|_| self.typ)
    }
    /// Returns the complete APS payload.
    #[must_use]
    pub const fn message(&self) -> &[u8] {
        &self.payload
    }
    /// Consumes the message and returns its complete APS payload.
    #[must_use]
    pub fn into_message(self) -> Box<[u8]> {
        self.payload
    }
    /// Returns the APS frame.
    #[must_use]
    pub const fn aps_frame(&self) -> &ApsFrame {
        &self.aps_frame
    }
    /// Returns the last-hop LQI.
    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }
    /// Returns the last-hop RSSI.
    #[must_use]
    pub const fn last_hop_rssi(&self) -> i8 {
        self.last_hop_rssi
    }
    /// Returns the sender.
    #[must_use]
    pub const fn sender(&self) -> NodeId {
        self.sender
    }
    /// Returns the binding index.
    #[must_use]
    pub const fn binding_index(&self) -> u8 {
        self.binding_index
    }
    /// Returns the address index.
    #[must_use]
    pub const fn address_index(&self) -> u8 {
        self.address_index
    }
    /// Returns the source-route overhead.
    #[must_use]
    pub const fn source_route_overhead(&self) -> Option<u8> {
        self.source_route_overhead
    }
    /// Returns the raw message type.
    #[must_use]
    pub const fn typ_value(&self) -> u8 {
        self.typ
    }

    pub(super) fn from_incoming_message(
        incoming_message: &IncomingMessage,
        payload: Box<[u8]>,
    ) -> Self {
        let mut aps_frame = incoming_message.aps_frame().clone();
        aps_frame.clear_fragmentation();

        Self {
            typ: incoming_message.typ_value(),
            aps_frame,
            last_hop_lqi: incoming_message.last_hop_lqi(),
            last_hop_rssi: incoming_message.last_hop_rssi(),
            sender: incoming_message.sender(),
            binding_index: incoming_message.binding_index(),
            address_index: incoming_message.address_index(),
            payload,
            source_route_overhead: incoming_message.source_route_overhead(),
        }
    }
}

impl From<IncomingMessage> for DefragmentedMessage {
    fn from(incoming_message: IncomingMessage) -> Self {
        let payload = incoming_message.message().into();
        Self::from_incoming_message(&incoming_message, payload)
    }
}
