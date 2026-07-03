use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::handler!(
    0x0032,
    { index: u8, policy_decision: u8 },
    impl {
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
    }
);
