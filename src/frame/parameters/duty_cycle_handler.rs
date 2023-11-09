use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberDutyCycleState};

pub const ID: u16 = 0x004D;

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
    channel_page: u8,
    channel: u8,
    state: EmberDutyCycleState,
    total_devices: u8,
}

impl Response {
    #[must_use]
    pub const fn new(channel_page: u8, channel: u8, state: EmberDutyCycleState, total_devices: u8) -> Self {
        Self { channel_page, channel, state, total_devices }
    }

    #[must_use]
    pub const fn channel_page(&self) -> u8 {
        self.channel_page
    }


    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }


    #[must_use]
    pub const fn state(&self) -> EmberDutyCycleState {
        self.state
    }


    #[must_use]
    pub const fn total_devices(&self) -> u8 {
        self.total_devices
    }
}
