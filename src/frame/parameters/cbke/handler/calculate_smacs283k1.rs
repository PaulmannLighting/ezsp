use le_stream::derive::FromLeStream;
use num_traits::FromPrimitive;

use super::Payload;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x00EB;

/// A callback to indicate that the NCP has finished calculating the
/// Secure Message Authentication Codes (`SMAC`) for both the initiator
/// and responder for the `CBKE` 283k1 Library.
///
/// The associated link key is kept in temporary storage until the host tells the NCP
/// to store or discard the key via `emberClearTemporaryDataMaybeStoreLinkKey()`.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    payload: Payload,
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Converts the handler into a [`Payload`] or an appropriate [`Error`] by evaluating its status field.
impl TryFrom<Handler> for Payload {
    type Error = Error;

    fn try_from(handler: Handler) -> Result<Self, Self::Error> {
        match Status::from_u8(handler.status).ok_or(handler.status) {
            Ok(Status::Success) => Ok(handler.payload),
            other => Err(other.into()),
        }
    }
}
