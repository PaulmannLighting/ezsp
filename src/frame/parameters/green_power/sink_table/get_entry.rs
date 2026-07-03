//! Parameters for the [`SinkTable::find_or_allocate_entry`](crate::SinkTable::find_or_allocate_entry) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::gp::sink::TableEntry;

crate::frame::parameters::frame!(
    0x00DD,
    { sink_index: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(sink_index: u8) -> Self {
                Self { sink_index }
            }
        }
    },
    { status: u8, entry: TableEntry },
    impl {
        /// Converts the response into a [`TableEntry`] or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for TableEntry {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                match Status::from_u8(response.status).ok_or(response.status) {
                    Ok(Status::Success) => Ok(response.entry),
                    other => Err(other.into()),
                }
            }
        }
    }
);
