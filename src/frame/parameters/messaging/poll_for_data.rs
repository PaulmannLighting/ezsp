//! Parameters for the [`Messaging::poll_for_data`](crate::Messaging::poll_for_data) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::event::Units;

crate::frame::parameters::frame!(
    0x0042,
    { interval: u16, units: u8, failure_limit: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub fn new(interval: u16, units: Units, failure_limit: u8) -> Self {
                Self {
                    interval,
                    units: units.into(),
                    failure_limit,
                }
            }
        }
    },
    { status: u8 },
    impl {
        /// Converts the response into `()` or an appropriate [`Error`] depending on its status.
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
