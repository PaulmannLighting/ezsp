use le_stream::derive::FromLeStream;

use crate::ember::zll::Network;
use crate::frame;
use crate::frame::Parameter;

const ID: u16 = 0x00BB;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    network_info: Network,
}

impl Handler {
    #[must_use]
    pub const fn network_info(&self) -> &Network {
        &self.network_info
    }
}

impl Parameter<frame::Extended<frame::Response>> for Handler {
    const ID: u16 = ID;
}
