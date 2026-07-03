//! Parameters for the [`Networking::get_source_route_table_entry`](crate::Networking::get_source_route_table_entry) command.

use le_stream::FromLeStream;
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{NodeId, Status};

crate::frame::parameters::frame!(0x00C1, { index: u8 }, { status: u8, entry: Entry });

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }
}

/// Convert a response into an [`Entry`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for Entry {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.entry),
            other => Err(other.into()),
        }
    }
}

/// A source route table entry.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Entry {
    destination: NodeId,
    closer_index: u8,
}

impl Entry {
    /// The node ID of the destination in that entry.
    #[must_use]
    pub const fn destination(&self) -> NodeId {
        self.destination
    }

    /// The closer node index for this source route table entry.
    #[must_use]
    pub const fn closer_index(&self) -> u8 {
        self.closer_index
    }
}
