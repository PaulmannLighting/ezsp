//! Returns information about a source route table entry.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::{NodeId, Status};
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x00C1;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub(crate) struct Response {
    status: u8,
    entry: Entry,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

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
