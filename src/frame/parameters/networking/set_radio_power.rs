//! Parameters for the [`Networking::set_radio_power`](crate::Networking::set_radio_power) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::frame!(
    0x0099,
    { power: i8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(power: i8) -> Self {
                Self { power }
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
