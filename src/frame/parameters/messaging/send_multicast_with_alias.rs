use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::aps::Frame;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::{Error, ValueError};

const ID: u16 = 0x003A;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    aps_frame: Frame,
    hops: u8,
    nonmember_radius: u8,
    alias: u16,
    nwk_sequence: u8,
    message_tag: u8,
    message_contents: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(
        aps_frame: Frame,
        hops: u8,
        nonmember_radius: u8,
        alias: u16,
        nwk_sequence: u8,
        message_tag: u8,
        message_contents: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            aps_frame,
            hops,
            nonmember_radius,
            alias,
            nwk_sequence,
            message_tag,
            message_contents,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    sequence: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

impl TryFrom<Response> for u8 {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        Status::from_u8(response.status)
            .ok_or_else(|| ValueError::Ember(response.status).into())
            .and_then(|status| {
                if status == Status::Success {
                    Ok(response.sequence)
                } else {
                    Err(status.into())
                }
            })
    }
}
