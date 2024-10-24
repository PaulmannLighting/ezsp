//! Stack Token Changed Handler

use le_stream::derive::FromLeStream;

use crate::frame::Identified;

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

impl Identified for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
