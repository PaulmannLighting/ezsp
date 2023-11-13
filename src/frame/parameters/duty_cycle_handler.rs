use crate::ember::DutyCycleState;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x004D;

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
    channel_page: u8,
    channel: u8,
    state: u8,
    total_devices: u8,
}

impl Response {
    #[must_use]
    pub const fn new(
        channel_page: u8,
        channel: u8,
        state: DutyCycleState,
        total_devices: u8,
    ) -> Self {
        Self {
            channel_page,
            channel,
            state: state.into(),
            total_devices,
        }
    }

    #[must_use]
    pub const fn channel_page(&self) -> u8 {
        self.channel_page
    }

    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    pub fn state(&self) -> Result<DutyCycleState, u8> {
        DutyCycleState::try_from(self.state)
    }

    #[must_use]
    pub const fn total_devices(&self) -> u8 {
        self.total_devices
    }
}
