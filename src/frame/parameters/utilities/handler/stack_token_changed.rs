use le_stream::derive::FromLeBytes;

use crate::frame::Parameter;

const ID: u16 = 0x000D;

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Handler {
    token_address: u16,
}

impl Handler {
    #[must_use]
    pub const fn token_address(&self) -> u16 {
        self.token_address
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
