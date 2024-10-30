use le_stream::derive::FromLeStream;
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x0032;

/// The NCP used the external binding modification policy to decide
/// how to handle a remote delete binding request.
///
/// The Host cannot change the current decision, but it can change the policy
/// for future decisions using the setPolicy command.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    index: u8,
    policy_decision: u8,
}

impl Identified for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert the handler into the index or an appropriate [`Error`]
/// by evaluating its policy decision field.
impl TryFrom<Handler> for u8 {
    type Error = Error;

    fn try_from(handler: Handler) -> Result<Self, Self::Error> {
        match Status::from_u8(handler.policy_decision).ok_or(handler.policy_decision) {
            Ok(Status::Success) => Ok(handler.index),
            other => Err(other.into()),
        }
    }
}
