use le_stream::FromLeStream;
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::frame::Parameter;

const ID: u16 = 0x0098;

/// A callback invoked by the `EmberZNet` stack when the MAC has finished transmitting a raw message.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
}

impl Parameter for Handler {
    const ID: u16 = ID;
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
