//! Parameters for the [`Messaging::get_multicast_table_entry`](crate::Messaging::get_multicast_table_entry) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::multicast::TableEntry;

crate::frame::parameters::frame!(
    0x0063,
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
    { status: u8, value: TableEntry } => Messaging(messaging)::GetMulticastTableEntry,
    impl {
        /// Converts the response into the [`TableEntry`] or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for TableEntry {
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
