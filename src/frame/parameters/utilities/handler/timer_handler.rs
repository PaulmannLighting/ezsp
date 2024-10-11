use le_stream::derive::FromLeStream;

use crate::frame::Parameter;

const ID: u16 = 0x000F;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    timer_id: u8,
}

impl Handler {
    #[must_use]
    pub const fn timer_id(&self) -> u8 {
        self.timer_id
    }
}

impl Parameter<u16> for Handler {
    const ID: u16 = ID;
}
