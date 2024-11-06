//! Parameters for the [`Configuration::set_policy`](crate::Configuration::set_policy) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ezsp::{decision, policy, Status};
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0055;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
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

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Converts the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
