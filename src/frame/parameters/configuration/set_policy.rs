use crate::error::Resolve;
use crate::ezsp::{decision, policy, Status};
use crate::frame::Parameter;
use crate::Error;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0055;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
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
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = ();

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve()
    }
}
