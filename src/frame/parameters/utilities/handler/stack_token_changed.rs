use crate::frame;
use crate::frame::Parameter;
use le_stream::derive::FromLeStream;

const ID: u16 = 0x000D;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    token_address: u16,
}

impl Handler {
    #[must_use]
    pub const fn token_address(&self) -> u16 {
        self.token_address
    }
}

impl Parameter<frame::Extended<frame::Response>> for Handler {
    const ID: u16 = ID;
}
