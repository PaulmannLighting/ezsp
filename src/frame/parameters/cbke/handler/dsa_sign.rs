use le_stream::derive::FromLeStream;
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::{Error, ValueError};

const ID: u16 = 0x00A7;

/// The handler that returns the results of the signing operation.
///
/// On success, the signature will be appended to the original message
/// (including the signature type indicator that replaced the startIndex field for the signing)
/// and both are returned via this callback.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    message: ByteSizedVec<u8>,
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

impl TryFrom<Handler> for ByteSizedVec<u8> {
    type Error = Error;

    fn try_from(handler: Handler) -> Result<Self, Self::Error> {
        Status::from_u8(handler.status)
            .ok_or_else(|| ValueError::Ember(handler.status).into())
            .and_then(|status| {
                if status == Status::Success {
                    Ok(handler.message)
                } else {
                    Err(status.into())
                }
            })
    }
}
