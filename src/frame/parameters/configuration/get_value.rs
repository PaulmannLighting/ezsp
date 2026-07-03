//! Parameters for the [`Configuration::get_value`](crate::Configuration::get_value) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ezsp::Status;
use crate::ezsp::value::Id;
use crate::types::ByteSizedVec;

crate::frame::parameters::frame!(
    0x00AA,
    { value_id: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub fn new(value_id: Id) -> Self {
                Self {
                    value_id: value_id.into(),
                }
            }
        }
    },
    { status: u8, value: ByteSizedVec<u8> } => Configuration(configuration)::GetValue,
    impl {
        /// Converts the response into an array of bytes or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for ByteSizedVec<u8> {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                match Status::from_u8(response.status).ok_or(response.status) {
                    Ok(Status::Success) => Ok(response.value),
                    other => Err(other.into()),
                }
            }
        }
    }
);
