use le_stream::derive::FromLeStream;

use crate::ember::Eui64;
use crate::frame::Parameter;

const ID: u16 = 0x0062;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    sender_eui64: Eui64,
}

impl Handler {
    #[must_use]
    pub const fn sender_eui64(&self) -> Eui64 {
        self.sender_eui64
    }
}

impl Parameter<u16> for Handler {
    const ID: u16 = ID;
}
