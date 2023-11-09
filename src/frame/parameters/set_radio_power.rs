use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{int8s,EmberStatus};

pub const ID: u16 = 0x0099;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    power: int8s,
}

impl Command {
    #[must_use]
    pub const fn new(power: int8s) -> Self {
        Self { power }
    }

    #[must_use]
    pub const fn power(&self) -> int8s {
        self.power
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
