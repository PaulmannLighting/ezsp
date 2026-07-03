//! Parameters for the [`SinkTable::set_entry`](crate::SinkTable::set_entry) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::gp::sink::TableEntry;

crate::frame::parameters::frame!(
    0x00DF,
    { sink_index: u8, entry: TableEntry },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(sink_index: u8, entry: TableEntry) -> Self {
                Self { sink_index, entry }
            }
        }
    },
    { status: u8 } => GreenPower(green_power)::SinkTable(sink_table)::SetEntry,
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
