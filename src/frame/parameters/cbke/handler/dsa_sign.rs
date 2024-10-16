use le_stream::derive::FromLeStream;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::resolve::Resolve;
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

impl Handler {
    /// The result of the DSA signing operation.
    ///
    /// # Returns
    ///
    /// The message and attached which includes the original message and the appended signature.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the status is not [`Status::Success`].
    pub fn result(&self) -> Result<&[u8], Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.message.as_slice())
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
