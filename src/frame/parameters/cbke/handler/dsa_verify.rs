use crate::ember::Status;
use crate::frame::Identified;
use crate::Error;
use le_stream::derive::FromLeStream;
use num_traits::FromPrimitive;

const ID: u16 = 0x0078;

/// This callback is executed by the stack when the DSA verification has completed and has a result.
///
/// If the result is [`Status::Success`], the signature is valid.
///
/// If the result is [`Status::SignatureVerifyFailure`] then the signature is invalid.
///
/// If the result is anything else then the signature verify operation failed and the validity is unknown.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
}

impl Handler {
    /// The result of the DSA verification operation.
    ///
    /// # Returns
    ///
    /// Returns `true` if the signature is valid, `false` if it failed.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the status is invalid.
    pub fn is_valid(&self) -> Result<bool, Error> {
        match Status::from_u8(self.status).ok_or(self.status) {
            Ok(Status::Success) => Ok(true),
            Ok(Status::SignatureVerifyFailure) => Ok(false),
            other => Err(other.into()),
        }
    }
}

impl Identified for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
