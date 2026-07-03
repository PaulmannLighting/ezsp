//! Parameters for the [`Utilities::set_mfg_token`](crate::Utilities::set_mfg_token) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ezsp::mfg_token::Id;
use crate::types::ByteSizedVec;

crate::frame::parameters::frame!(
    0x000C,
    { token_id: u8, token_data: ByteSizedVec<u8> },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub fn new(token_id: Id, token_data: ByteSizedVec<u8>) -> Self {
                Self {
                    token_id: token_id.into(),
                    token_data,
                }
            }
        }
    },
    { status: u8 },
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
