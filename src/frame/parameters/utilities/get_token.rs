//! Parameters for the [`Utilities::get_token`](crate::Utilities::get_token) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::frame!(
    0x000A,
    { token_id: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(token_id: u8) -> Self {
                Self { token_id }
            }
        }
    },
    { status: u8, token_data: [u8; 8] },
    impl {
        /// Convert the response into an array of bytes or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for [u8; 8] {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                match Status::from_u8(response.status).ok_or(response.status) {
                    Ok(Status::Success) => Ok(response.token_data),
                    other => Err(other.into()),
                }
            }
        }
    }
);
