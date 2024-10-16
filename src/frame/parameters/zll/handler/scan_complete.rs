use le_stream::derive::FromLeStream;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::resolve::Resolve;

const ID: u16 = 0x00B7;

/// This call is fired when a ZLL network scan is complete.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
}

impl Handler {
    /// Status of the operation.
    ///
    /// # Errors
    ///
    /// Returns an error if the status is not success.
    pub fn result(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
