//! Stack Token Changed Handler

use le_stream::FromLeStream;

use crate::frame::Parameter;

const ID: u16 = 0x000D;

/// A callback invoked to inform the application that a stack token has changed.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    token_address: u16,
}

impl Handler {
    /// The address of the stack token that has changed.
    #[must_use]
    pub const fn token_address(&self) -> u16 {
        self.token_address
    }
}

impl Parameter for Handler {
    const ID: u16 = ID;
}
