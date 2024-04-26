use le_stream::derive::FromLeBytes;

use crate::ember::duty_cycle::State;
use crate::frame::Parameter;

const ID: u16 = 0x004D;

#[derive(Debug, Eq, PartialEq, FromLeBytes)]
pub struct Handler {
    channel_page: u8,
    channel: u8,
    state: u8,
    total_devices: u8,
}

impl Handler {
    #[must_use]
    pub const fn channel_page(&self) -> u8 {
        self.channel_page
    }

    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    pub fn state(&self) -> Result<State, u8> {
        State::try_from(self.state)
    }

    #[must_use]
    pub const fn total_devices(&self) -> u8 {
        self.total_devices
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
