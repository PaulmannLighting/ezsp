use le_stream::derive::FromLeStream;
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Error;

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
    const ID: Self::Id = ID;
}

impl TryFrom<Handler> for ByteSizedVec<u8> {
    type Error = Error;

    fn try_from(handler: Handler) -> Result<Self, Self::Error> {
        match Status::from_u8(handler.status).ok_or(handler.status) {
            Ok(Status::Success) => Ok(handler.message),
            other => Err(other.into()),
        }
    }
}
