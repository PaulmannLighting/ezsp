use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::entropy::Source;
use crate::frame::Identified;
use crate::{Error, ValueError};

const ID: u16 = 0x004F;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    entropy_source: u8,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Response> for Source {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, <Self as TryFrom<Response>>::Error> {
        Self::from_u8(response.entropy_source)
            .ok_or_else(|| ValueError::EntropySource(response.entropy_source).into())
    }
}
