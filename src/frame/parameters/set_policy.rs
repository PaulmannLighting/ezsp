use crate::ezsp::{decision, policy, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0055;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    policy_id: u8,
    decision_id: u8,
}

impl Command {
    #[must_use]
    pub fn new(policy_id: policy::Id, decision_id: decision::Id) -> Self {
        Self {
            policy_id: policy_id.into(),
            decision_id: decision_id.into(),
        }
    }

    pub fn policy_id(&self) -> Result<policy::Id, u8> {
        policy::Id::try_from(self.policy_id)
    }

    pub fn decision_id(&self) -> Result<decision::Id, u8> {
        decision::Id::try_from(self.decision_id)
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
