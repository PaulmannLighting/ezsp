//! Parameters for the [`Zll::set_rx_on_when_idle`](crate::Zll::set_rx_on_when_idle) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::frame!(
    0x00B5,
    { duration_ms: u32 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(duration_ms: u32) -> Self {
                Self { duration_ms }
            }
        }
    },
    { status: u8 },
    impl {
        /// Convert the response into a [`Result<()>`](crate::Result) by evaluating its status field.
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
