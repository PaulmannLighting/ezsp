//! Parameters for the [`Networking::get_neighbor`](crate::Networking::get_neighbor) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::neighbor::TableEntry;

crate::frame::parameters::frame!(
    0x0079,
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
    { status: u8, value: TableEntry } => Networking(networking)::GetNeighbor,
    impl {
        /// Convert a response into a [`TableEntry`] or an appropriate [`Error`] depending on its status.
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
