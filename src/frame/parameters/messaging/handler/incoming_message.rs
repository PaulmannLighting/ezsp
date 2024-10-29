use le_stream::derive::FromLeStream;
use num_traits::FromPrimitive;

use crate::ember::aps::Frame;
use crate::ember::message::Incoming;
use crate::ember::node::Type;
use crate::ember::NodeId;
use crate::frame::Identified;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0045;

/// A callback indicating a message has been received.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    typ: u8,
    aps_frame: Frame,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
    sender: NodeId,
    binding_index: u8,
    address_index: u8,
    message: ByteSizedVec<u8>,
    // FIXME: There appears to be one byte more at the end than specified in the docs in most cases.
    // Assume optional node type for now.
    node_type: Option<u8>,
}

impl Handler {
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
        &self.message
    }

    /// The type of the sender node.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not a valid node type.
    #[must_use]
    pub fn node_type(&self) -> Option<Type> {
        self.node_type.and_then(Type::from_u8)
    }
}

impl Identified for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
