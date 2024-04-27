use le_stream::derive::FromLeBytes;

use crate::frame::Parameter;

const ID: u16 = 0x000F;

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Handler {
    timer_id: u8,
}

impl Handler {
    #[must_use]
    pub const fn timer_id(&self) -> u8 {
        self.timer_id
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
