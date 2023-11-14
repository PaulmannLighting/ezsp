use crate::ember::beacon::Data;
use crate::ember::node::Type;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x003B;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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

    pub fn local_node_type(&self) -> Result<Type, u8> {
        Type::try_from(self.local_node_type)
    }

    #[must_use]
    pub const fn beacon(&self) -> &Data {
        &self.beacon
    }

    #[must_use]
    pub const fn radio_tx_power(&self) -> i8 {
        self.radio_tx_power
    }

    #[must_use]
    pub const fn clear_beacons_after_network_up(&self) -> bool {
        self.clear_beacons_after_network_up
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
