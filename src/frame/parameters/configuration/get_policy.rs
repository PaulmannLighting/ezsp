use crate::ezsp::Status;
use crate::ezsp::{decision, policy};
use crate::frame::Parameter;
use crate::Resolve;
use crate::{Error, ValueError};
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0056;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
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
}

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    decision_id: u8,
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = decision::Id;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve().and_then(|_| {
            decision::Id::try_from(self.decision_id)
                .map_err(|id| Error::ValueError(ValueError::InvalidDecisionId(id)))
        })
    }
}
