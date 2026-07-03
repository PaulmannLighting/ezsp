//! Parameters for the [`Networking::get_child_data`](crate::Networking::get_child_data) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::child::Data;

crate::frame::parameters::frame!(
    0x004A,
    { index: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(index: u8) -> Self {
                Self { index }
            }
        }
    },
    { status: u8, child_data: Data },
    impl {
        /// Converts the response into [`Data`] or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for Data {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                match Status::from_u8(response.status).ok_or(response.status) {
                    Ok(Status::Success) => Ok(response.child_data),
                    other => Err(other.into()),
                }
            }
        }
    }
);
