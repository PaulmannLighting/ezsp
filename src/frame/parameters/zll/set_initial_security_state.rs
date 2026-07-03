//! Parameters for the [`Zll::set_initial_security_state`](crate::Zll::set_initial_security_state) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::key::Data;
use crate::ember::zll::InitialSecurityState;

crate::frame::parameters::frame!(
    0x00B3,
    { network_key: Data, security_state: InitialSecurityState },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(network_key: Data, security_state: InitialSecurityState) -> Self {
                Self {
                    network_key,
                    security_state,
                }
            }
        }
    },
    { status: u8 },
    impl {
        /// Convert the response into a [`Result<()>`](crate::Result) by evaluating its status field.
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
