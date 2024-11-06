//! Counter Rollover Handler

use le_stream::derive::FromLeStream;

use crate::ember::counter::Type;
use crate::frame::Parameter;

const ID: u16 = 0x00F2;

/// This call is fired when a counter exceeds its threshold.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    typ: u8,
}

impl Handler {
    /// Type of Counter.
    ///
    /// # Errors
    ///
    /// Returns an error if the counter type is invalid.
    pub fn typ(&self) -> Result<Type, u8> {
        Type::try_from(self.typ)
    }
}

impl Parameter for Handler {
    const ID: u16 = ID;
}
