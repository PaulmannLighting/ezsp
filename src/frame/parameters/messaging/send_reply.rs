//! Parameters for the [`Messaging::send_reply`](crate::Messaging::send_reply) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::aps::Frame;
use crate::ember::{NodeId, Status};
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Error;

const ID: u16 = 0x0039;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    sender: NodeId,
    aps_frame: Frame,
    message: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(sender: NodeId, aps_frame: Frame, message: ByteSizedVec<u8>) -> Self {
        Self {
            sender,
            aps_frame,
            message,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

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
