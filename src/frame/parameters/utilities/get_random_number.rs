//! Parameters for the [`Utilities::get_random_number`](crate::Utilities::get_random_number) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::frame!(
    0x0049,
    {},
    { status: u8, value: u16 } => Utilities(utilities)::GetRandomNumber,
    impl {
        /// Convert the response into the generated random number
        /// or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for u16 {
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
