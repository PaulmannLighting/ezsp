use crate::ember::NodeId;
use crate::ember::aps::Frame;
use crate::ember::message::Incoming;
use crate::parameters::messaging::handler::IncomingMessage;

/// A message that has been defragmented and is ready for processing.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct DefragmentedMessage {
    typ: u8,
    aps_frame: Frame,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
    sender: NodeId,
    binding_index: u8,
    address_index: u8,
    message: Vec<u8>,
}

impl DefragmentedMessage {
    /// Create a new `DefragmentedMessage` from an `IncomingMessage`.
    pub(crate) fn new(incoming_message: IncomingMessage) -> Self {
        Self {
            typ: incoming_message.typ,
            aps_frame: incoming_message.aps_frame,
            last_hop_lqi: incoming_message.last_hop_lqi,
            last_hop_rssi: incoming_message.last_hop_rssi,
            sender: incoming_message.sender,
            binding_index: incoming_message.binding_index,
            address_index: incoming_message.address_index,
            message: incoming_message.message.into_data().drain(..).collect(),
        }
    }

    /// Create a new `DefragmentedMessage` with the specified first part and complete message.
    pub(crate) fn new_with_message(first: IncomingMessage, message: Vec<u8>) -> Self {
        Self {
            typ: first.typ,
            aps_frame: first.aps_frame,
            last_hop_lqi: first.last_hop_lqi,
            last_hop_rssi: first.last_hop_rssi,
            sender: first.sender,
            binding_index: first.binding_index,
            address_index: first.address_index,
            message,
        }
    }

    /// The type of the incoming message.
    ///
    /// One of the following:
    ///
    /// - [`Incoming::Unicast`]
    /// - [`Incoming::UnicastReply`]
    /// - [`Incoming::Multicast`]
    /// - [`Incoming::MulticastLoopback`]
    /// - [`Incoming::Broadcast`]
    /// - [`Incoming::BroadcastLoopback`]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not a valid incoming message type.
    pub fn typ(&self) -> Result<Incoming, u8> {
        Incoming::try_from(self.typ)
    }

    /// The APS frame from the incoming message.
    #[must_use]
    pub const fn aps_frame(&self) -> &Frame {
        &self.aps_frame
    }

    /// The link quality from the node that last relayed the message.
    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }

    /// The energy level (in units of dBm) observed during the reception.
    #[must_use]
    pub const fn last_hop_rssi(&self) -> i8 {
        self.last_hop_rssi
    }

    /// The sender of the message.
    #[must_use]
    pub const fn sender(&self) -> NodeId {
        self.sender
    }

    /// The index of a binding that matches the message or 0xFF if there is no matching binding.
    #[must_use]
    pub const fn binding_index(&self) -> u8 {
        self.binding_index
    }

    /// The index of the entry in the address table that matches the sender
    /// of the message or 0xFF if there is no matching entry.
    #[must_use]
    pub const fn address_index(&self) -> u8 {
        self.address_index
    }

    /// The incoming message.
    #[must_use]
    pub fn message(&self) -> &[u8] {
        self.message.as_ref()
    }

    /// Consumes the handler and returns the incoming message.
    #[must_use]
    pub fn into_message(self) -> Vec<u8> {
        self.message
    }
}
