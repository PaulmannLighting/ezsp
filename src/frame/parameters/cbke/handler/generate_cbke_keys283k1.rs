use le_stream::derive::FromLeStream;
use num_traits::FromPrimitive;

use crate::ember::{PublicKey283k1Data, Status};
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x00E9;

/// A callback by the Crypto Engine indicating that a new
/// 283k1 ephemeral public/private key pair has been generated.
///
/// The public/private key pair is stored on the NCP, but
/// only the associated public key is returned to the host.
///
/// The node's associated certificate is also returned.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    ephemeral_public_key: PublicKey283k1Data,
}

impl Parameter for Handler {
    const ID: u16 = ID;
}

/// Converts the handler into a [`PublicKey283k1Data`] or an appropriate [`Error`]
/// by evaluating its status field.
impl TryFrom<Handler> for PublicKey283k1Data {
    type Error = Error;

    fn try_from(handler: Handler) -> Result<Self, Self::Error> {
        match Status::from_u8(handler.status).ok_or(handler.status) {
            Ok(Status::Success) => Ok(handler.ephemeral_public_key),
            other => Err(other.into()),
        }
    }
}
