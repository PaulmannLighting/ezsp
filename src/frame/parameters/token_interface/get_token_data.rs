//! Parameters for the [`TokenInterface::get_token_data`](crate::TokenInterface::get_token_data) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::token::Data;

crate::frame::parameters::frame!(
    0x0102,
    { token: u32, index: u32 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(token: u32, index: u32) -> Self {
                Self { token, index }
            }
        }
    },
    { status: u8, token_data: Data } => TokenInterface(token_interface)::GetTokenData,
    impl {
        /// Convert the response into [`Data`] or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for Data {
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
