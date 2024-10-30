//! Parameters for then [`Networking::set_neighbor_frame_counter`](crate::Networking::set_neighbor_frame_counter) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::{Eui64, Status};
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x00AD;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    eui64: Eui64,
    frame_counter: u32,
}

impl Command {
    #[must_use]
    pub const fn new(eui64: Eui64, frame_counter: u32) -> Self {
        Self {
            eui64,
            frame_counter,
        }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
