//! Parameters for the [`Utilities::set_timer`](crate::Utilities::set_timer) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::event::Units;

crate::frame::parameters::frame!(
    0x000E,
    { timer_id: u8, time: u16, units: u8, repeat: bool },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub fn new(timer_id: u8, time: u16, units: Units, repeat: bool) -> Self {
                Self {
                    timer_id,
                    time,
                    units: units.into(),
                    repeat,
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
