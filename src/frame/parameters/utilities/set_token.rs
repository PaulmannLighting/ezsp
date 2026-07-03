//! Parameters for the [`Utilities::set_token`](crate::Utilities::set_token) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::frame!(
    0x0009,
    { token_id: u8, token_data: [u8; 8] },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(token_id: u8, token_data: [u8; 8]) -> Self {
                Self {
                    token_id,
                    token_data,
                }
            }
        }
    },
    { status: u8 } => Utilities(utilities)::SetToken,
    impl {
        /// Convert the response into `()` or an appropriate [`Error`] depending on its status.
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
