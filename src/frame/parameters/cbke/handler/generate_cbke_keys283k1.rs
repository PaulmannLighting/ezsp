use le_stream::derive::FromLeStream;
use num_traits::FromPrimitive;

use crate::ember::{PublicKey283k1Data, Status};
use crate::frame::Parameter;
use crate::Error;
use crate::ValueError;

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
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

impl TryFrom<Handler> for PublicKey283k1Data {
    type Error = Error;

    fn try_from(handler: Handler) -> Result<Self, Self::Error> {
        Status::from_u8(handler.status)
            .ok_or_else(|| ValueError::Ember(handler.status).into())
            .and_then(|status| {
                if status == Status::Success {
                    Ok(handler.ephemeral_public_key)
                } else {
                    Err(status.into())
                }
            })
    }
}
