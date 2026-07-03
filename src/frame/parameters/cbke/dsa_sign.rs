//! Parameters for the [`Cbke::dsa_sign`](crate::Cbke::dsa_sign) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::types::ByteSizedVec;

crate::frame::parameters::frame!(
    0x00A6,
    { message: ByteSizedVec<u8> },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(message: ByteSizedVec<u8>) -> Self {
                Self { message }
            }
        }
    },
    { status: u8 } => Cbke(cbke)::DsaSign,
    impl {
        /// Converts the response into `()` or an appropriate [`Error`] by evaluating its status field.
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
