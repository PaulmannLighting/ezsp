//! Timer handler parameter.

use le_stream::derive::FromLeStream;

use crate::frame::Parameter;

const ID: u16 = 0x000F;

/// A callback from the timer.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    timer_id: u8,
}

impl Handler {
    /// Which timer generated the callback (0 or 1).
    #[must_use]
    pub const fn timer_id(&self) -> u8 {
        self.timer_id
    }
}

impl Parameter for Handler {
    const ID: u16 = ID;
}
