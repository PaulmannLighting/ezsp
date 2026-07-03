//! Parameters for the [`Security::check_key_context`](crate::Security::check_key_context) command.

use num_traits::FromPrimitive;
use silizium::Status;
use silizium::zigbee::security::man::Context;

use crate::Error;

crate::frame::parameters::frame!(
    0x0110,
    { context: Context },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(context: Context) -> Self {
                Self { context }
            }
        }
    },
    { status: u32 },
    impl {
        /// Convert the response into `()` or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for () {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                match Status::from_u32(response.status).ok_or(response.status) {
                    Ok(Status::Ok) => Ok(()),
                    other => Err(other.into()),
                }
            }
        }
    }
);
