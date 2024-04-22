use crate::ember::PanId;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00D2;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    pan_id: PanId,
    channel: u8,
}

impl Response {
    #[must_use]
    pub const fn new(pan_id: PanId, channel: u8) -> Self {
        Self { pan_id, channel }
    }

    #[must_use]
    pub const fn pan_id(&self) -> PanId {
        self.pan_id
    }

    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }
}
