//! Parameters for the [`Configuration::set_policy`](crate::Configuration::set_policy) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ezsp::{Status, policy};

crate::frame::parameters::frame!(
    0x0055,
    { policy_id: u8, decision_id: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub fn new(policy_id: policy::Id, decision_id: u8) -> Self {
                Self {
                    policy_id: policy_id.into(),
                    decision_id,
                }
            }
        }
    },
    { status: u8 } => Configuration(configuration)::SetPolicy,
    impl {
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
    }
);
