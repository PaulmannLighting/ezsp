use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::beacon::Data;
use crate::ember::node::Type;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x003B;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    local_node_type: u8,
    beacon: Data,
    radio_tx_power: i8,
    clear_beacons_after_network_up: bool,
}

impl Command {
    #[must_use]
    pub fn new(
        local_node_type: Type,
        beacon: Data,
        radio_tx_power: i8,
        clear_beacons_after_network_up: bool,
    ) -> Self {
        Self {
            local_node_type: local_node_type.into(),
            beacon,
            radio_tx_power,
            clear_beacons_after_network_up,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = ();

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve()
    }
}
