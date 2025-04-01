//! Parameters for the [`Configuration::set_policy`](crate::Configuration::set_policy) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::error::ValueError;
use crate::ezsp::Status;
use crate::ezsp::{decision, policy};
use crate::frame::Parameter;

const ID: u16 = 0x0056;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub(crate) struct Command {
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

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    decision_id: u8,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Converts the response into a [`decision::Id`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for decision::Id {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Self::from_u8(response.decision_id)
                .ok_or_else(|| ValueError::DecisionId(response.decision_id).into()),
            other => Err(other.into()),
        }
    }
}
