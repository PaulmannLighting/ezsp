//! Parameters for the [`Messaging::set_multicast_table_entry`](crate::Messaging::set_multicast_table_entry) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::multicast::TableEntry;

crate::frame::parameters::frame!(
    0x0064,
    { index: u8, value: TableEntry },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(index: u8, value: TableEntry) -> Self {
                Self { index, value }
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
