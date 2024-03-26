use crate::ember::node::Type;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00B4;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    channel_mask: u32,
    radio_power_for_scan: i8,
    node_type: u8,
}

impl Command {
    #[must_use]
    pub fn new(channel_mask: u32, radio_power_for_scan: i8, node_type: Type) -> Self {
        Self {
            channel_mask,
            radio_power_for_scan,
            node_type: node_type.into(),
        }
    }

    #[must_use]
    pub const fn channel_mask(&self) -> u32 {
        self.channel_mask
    }

    #[must_use]
    pub const fn radio_power_for_scan(&self) -> i8 {
        self.radio_power_for_scan
    }

    pub fn node_type(&self) -> Result<Type, u8> {
        Type::try_from(self.node_type)
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
