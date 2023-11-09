use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberStatus};

pub const ID: u16 = 0x0032;

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
    index: u8,
    policy_decision: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(index: u8, policy_decision: EmberStatus) -> Self {
        Self { index, policy_decision }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }


    #[must_use]
    pub const fn policy_decision(&self) -> EmberStatus {
        self.policy_decision
    }
}
