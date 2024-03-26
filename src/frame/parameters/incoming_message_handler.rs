use crate::ember::aps::Frame;
use crate::ember::message::Incoming;
use crate::ember::NodeId;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0045;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    typ: u8,
    aps_frame: Frame,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
    sender: NodeId,
    binding_index: u8,
    address_index: u8,
    message: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub fn new(
        typ: Incoming,
        aps_frame: Frame,
        last_hop_lqi: u8,
        last_hop_rssi: i8,
        sender: NodeId,
        binding_index: u8,
        address_index: u8,
        message: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            typ: typ.into(),
            aps_frame,
            last_hop_lqi,
            last_hop_rssi,
            sender,
            binding_index,
            address_index,
            message,
        }
    }

    pub fn typ(&self) -> Result<Incoming, u8> {
        Incoming::try_from(self.typ)
    }

    #[must_use]
    pub const fn aps_frame(&self) -> &Frame {
        &self.aps_frame
    }

    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }

    #[must_use]
    pub const fn last_hop_rssi(&self) -> i8 {
        self.last_hop_rssi
    }

    #[must_use]
    pub const fn sender(&self) -> NodeId {
        self.sender
    }

    #[must_use]
    pub const fn binding_index(&self) -> u8 {
        self.binding_index
    }

    #[must_use]
    pub const fn address_index(&self) -> u8 {
        self.address_index
    }

    #[must_use]
    pub const fn message(&self) -> &ByteSizedVec<u8> {
        &self.message
    }
}
