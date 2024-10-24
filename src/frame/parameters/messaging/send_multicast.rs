use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::aps::Frame;
use crate::ember::Status;
use crate::frame::Identified;
use crate::types::ByteSizedVec;
use crate::Error;

const ID: u16 = 0x0038;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    aps_frame: Frame,
    hops: u8,
    nonmember_radius: u8,
    message_tag: u8,
    message_contents: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(
        aps_frame: Frame,
        hops: u8,
        nonmember_radius: u8,
        message_tag: u8,
        message_contents: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            aps_frame,
            hops,
            nonmember_radius,
            message_tag,
            message_contents,
        }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    sequence: u8,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Response> for u8 {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.sequence),
            other => Err(other.into()),
        }
    }
}
