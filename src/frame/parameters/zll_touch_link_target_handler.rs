use crate::ember::zll::Network;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00BB;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    network_info: Network,
}

impl Response {
    #[must_use]
    pub const fn new(network_info: Network) -> Self {
        Self { network_info }
    }

    #[must_use]
    pub const fn network_info(&self) -> &Network {
        &self.network_info
    }
}
