use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberZigbeeNetwork,int8s};

pub const ID: u16 = 0x001B;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {  }
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    network_found: EmberZigbeeNetwork,
    last_hop_lqi: u8,
    last_hop_rssi: int8s,
}

impl Response {
    #[must_use]
    pub const fn new(network_found: EmberZigbeeNetwork, last_hop_lqi: u8, last_hop_rssi: int8s) -> Self {
        Self { network_found, last_hop_lqi, last_hop_rssi }
    }

    #[must_use]
    pub const fn network_found(&self) -> EmberZigbeeNetwork {
        self.network_found
    }


    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }


    #[must_use]
    pub const fn last_hop_rssi(&self) -> int8s {
        self.last_hop_rssi
    }
}
