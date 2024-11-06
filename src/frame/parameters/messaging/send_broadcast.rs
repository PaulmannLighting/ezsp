//! Parameters for the [`Messaging::send_broadcast`](crate::Messaging::send_broadcast) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::aps::Frame;
use crate::ember::{NodeId, Status};
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Error;

const ID: u16 = 0x0036;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    destination: NodeId,
    aps_frame: Frame,
    radius: u8,
    message_tag: u8,
    message_contents: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(
        destination: NodeId,
        aps_frame: Frame,
        radius: u8,
        message_tag: u8,
        message_contents: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            destination,
            aps_frame,
            radius,
            message_tag,
            message_contents,
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
    sequence: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Converts the response into the sequence number or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for u8 {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.sequence),
            other => Err(other.into()),
        }
    }
}
