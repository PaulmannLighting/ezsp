//! Parameters for the [`Messaging::set_address_table_remote_eui64`](crate::Messaging::set_address_table_remote_eui64) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{Eui64, Status};

crate::frame::parameters::frame!(
    0x005C,
    { address_table_index: u8, eui64: Eui64 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(address_table_index: u8, eui64: Eui64) -> Self {
                Self {
                    address_table_index,
                    eui64,
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
