use le_stream::derive::FromLeStream;
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::{Error, ValueError};

const ID: u16 = 0x00C7;

/// A callback to the GP endpoint to indicate the result of the GPDF transmission.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    gpep_handle: u8,
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

impl TryFrom<Handler> for u8 {
    type Error = Error;

    fn try_from(handler: Handler) -> Result<Self, Self::Error> {
        Status::from_u8(handler.status)
            .ok_or_else(|| ValueError::Ember(handler.status).into())
            .and_then(|status| {
                if status == Status::Success {
                    Ok(handler.gpep_handle)
                } else {
                    Err(status.into())
                }
            })
    }
}
