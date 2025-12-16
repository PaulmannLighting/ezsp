use le_stream::FromLeStream;

use crate::ember::Eui64;
use crate::ember::key::{Status, TrustCenter};
use crate::frame::Parameter;

const ID: u16 = 0x009B;

/// This is a callback that indicates the success or failure of an attempt
/// to establish a key with a partner device.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    // This value is all zeros on failure according to the documentation.
    // Thus, we wil return an option in the appropriate getter.
    partner: Eui64,
    status: u8,
}

impl Handler {
    /// This is the IEEE address of the partner that the device successfully established a key with.
    ///
    /// # Errors
    ///
    /// Returns an error if the key establishment was not successful, containing either the status
    /// indicating what was established or why the key establishment failed.
    pub fn partner(&self) -> Result<Eui64, Result<Status, u8>> {
        match self.status() {
            Ok(
                Status::AppLinkKeyEstablished
                | Status::TrustCenterLinkKeyEstablished
                | Status::TrustCenter(TrustCenter::RespondedToKeyRequest),
            ) => Ok(self.partner),
            Ok(other) => Err(Ok(other)),
            Err(invalid) => Err(Err(invalid)),
        }
    }

    /// This is the status indicating what was established or why the key establishment failed.
    ///
    /// # Errors
    ///
    /// Returns an error if the status is invalid.
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}

impl Parameter for Handler {
    const ID: u16 = ID;
}
