use le_stream::derive::FromLeStream;

use crate::ember::{PublicKeyData, Status};
use crate::frame::Parameter;
use crate::resolve::Resolve;
use crate::Error;

const ID: u16 = 0x009E;

/// A callback by the Crypto Engine indicating that a new
/// ephemeral public/private key pair has been generated.
///
/// The public/private key pair is stored on the NCP, but only the associated public key
/// is returned to the host.
///
/// The node's associated certificate is also returned.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    ephemeral_public_key: PublicKeyData,
}

impl Handler {
    /// The result of the CBKE operation.
    ///
    /// # Returns
    ///
    /// The generated ephemeral public key.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the status is not [`Status::Success`].
    pub fn result(&self) -> Result<PublicKeyData, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.ephemeral_public_key)
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
