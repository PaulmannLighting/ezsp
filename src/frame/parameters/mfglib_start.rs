use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{bool,EmberStatus};

pub const ID: u16 = 0x0083;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    rx_callback: bool,
}

impl Command {
    #[must_use]
    pub const fn new(rx_callback: bool) -> Self {
        Self { rx_callback }
    }

    #[must_use]
    pub const fn rx_callback(&self) -> bool {
        self.rx_callback
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
