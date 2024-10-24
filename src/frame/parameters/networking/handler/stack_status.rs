use le_stream::derive::FromLeStream;
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::frame::Identified;

const ID: u16 = 0x0019;

/// A callback invoked when the status of the stack changes.
///
/// If the status parameter equals [`Status::NetworkUp`], then the getNetworkParameters command
/// can be called to obtain the new network parameters.
///
/// If any of the parameters are being stored in nonvolatile memory by the Host,
/// the stored values should be updated.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
}

impl Handler {
    /// Stack status.
    ///
    /// # Errors
    ///
    /// Returns an error if the status is invalid.
    pub fn result(&self) -> Result<Status, u8> {
        Status::from_u8(self.status).ok_or(self.status)
    }
}

impl Identified for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
