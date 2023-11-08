use crate::types::{int8s, EmberApsFrame, EmberIncomingMessageType, EmberNodeId};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0045;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    typ: EmberIncomingMessageType,
    aps_frame: EmberApsFrame,
    last_hop_lqi: u8,
    last_hop_rssi: int8s,
    sender: EmberNodeId,
    binding_index: u8,
    address_index: u8,
    message_length: u8,
    message_contents: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub const fn new(
        typ: EmberIncomingMessageType,
        aps_frame: EmberApsFrame,
        last_hop_lqi: u8,
        last_hop_rssi: int8s,
        sender: EmberNodeId,
        binding_index: u8,
        address_index: u8,
        message_length: u8,
        message_contents: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            typ,
            aps_frame,
            last_hop_lqi,
            last_hop_rssi,
            sender,
            binding_index,
            address_index,
            message_length,
            message_contents,
        }
    }

    #[must_use]
    pub const fn typ(&self) -> EmberIncomingMessageType {
        self.typ
    }

    #[must_use]
    pub const fn aps_frame(&self) -> EmberApsFrame {
        self.aps_frame
    }

    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }

    #[must_use]
    pub const fn last_hop_rssi(&self) -> int8s {
        self.last_hop_rssi
    }

    #[must_use]
    pub const fn sender(&self) -> EmberNodeId {
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
    pub const fn message_length(&self) -> u8 {
        self.message_length
    }

    #[must_use]
    pub const fn message_contents(&self) -> ByteSizedVec<u8> {
        self.message_contents
    }
}
