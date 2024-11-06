use le_stream::derive::FromLeStream;
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0043;

/// Indicates the result of a data poll to the parent of the local node.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Handler> for () {
    type Error = Error;

    fn try_from(handler: Handler) -> Result<Self, Self::Error> {
        match Status::from_u8(handler.status).ok_or(handler.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
