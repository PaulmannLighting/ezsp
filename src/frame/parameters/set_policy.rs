use crate::ezsp::{decision, policy, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0055;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    policy_id: policy::Id,
    decision_id: decision::Id,
}

impl Command {
    #[must_use]
    pub const fn new(policy_id: policy::Id, decision_id: decision::Id) -> Self {
        Self {
            policy_id,
            decision_id,
        }
    }

    #[must_use]
    pub const fn policy_id(&self) -> policy::Id {
        self.policy_id
    }

    #[must_use]
    pub const fn decision_id(&self) -> decision::Id {
        self.decision_id
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
