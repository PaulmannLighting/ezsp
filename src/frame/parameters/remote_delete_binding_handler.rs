use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0032;

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
    index: u8,
    policy_decision: u8,
}

impl Response {
    #[must_use]
    pub fn new(index: u8, policy_decision: Status) -> Self {
        Self {
            index,
            policy_decision: policy_decision.into(),
        }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    pub fn policy_decision(&self) -> Result<Status, u8> {
        Status::try_from(self.policy_decision)
    }
}
