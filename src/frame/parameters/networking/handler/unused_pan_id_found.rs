use le_stream::derive::FromLeStream;

use crate::ember::PanId;
use crate::frame::Parameter;

const ID: u16 = 0x00D2;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    pan_id: PanId,
    channel: u8,
}

impl Handler {
    #[must_use]
    pub const fn pan_id(&self) -> PanId {
        self.pan_id
    }

    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }
}

impl Parameter<u16> for Handler {
    const ID: u16 = ID;
}
