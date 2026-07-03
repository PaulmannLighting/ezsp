//! Parameters for the [`Security::get_current_security_state`](crate::Security::get_current_security_state) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::security::current::State;

crate::frame::parameters::frame!(
    0x0069,
    {},
    { status: u8, state: State },
    impl {
        /// Convert the response into [`State`] or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for State {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                match Status::from_u8(response.status).ok_or(response.status) {
                    Ok(Status::Success) => Ok(response.state),
                    other => Err(other.into()),
                }
            }
        }
    }
);
