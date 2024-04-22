use crate::ezsp::Status;
use crate::ezsp::{decision, policy};
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0056;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    policy_id: u8,
}

impl Command {
    #[must_use]
    pub fn new(policy_id: policy::Id) -> Self {
        Self {
            policy_id: policy_id.into(),
        }
    }

    pub fn policy_id(&self) -> Result<policy::Id, u8> {
        policy::Id::try_from(self.policy_id)
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    decision_id: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, decision_id: decision::Id) -> Self {
        Self {
            status: status.into(),
            decision_id: decision_id.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    pub fn decision_id(&self) -> Result<decision::Id, u8> {
        decision::Id::try_from(self.decision_id)
    }
}
