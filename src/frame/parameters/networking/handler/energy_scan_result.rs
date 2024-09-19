use le_stream::derive::FromLeStream;

use crate::frame::Parameter;

const ID: u16 = 0x0048;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    channel: u8,
    max_rssi_value: i8,
}

impl Handler {
    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    #[must_use]
    pub const fn max_rssi_value(&self) -> i8 {
        self.max_rssi_value
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
