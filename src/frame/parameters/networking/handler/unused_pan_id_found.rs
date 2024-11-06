use le_stream::derive::FromLeStream;

use crate::ember::PanId;
use crate::frame::Parameter;

const ID: u16 = 0x00D2;

/// This function returns an unused panID and channel pair found
/// via the find unused panId scan procedure.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    pan_id: PanId,
    channel: u8,
}

impl Handler {
    /// The unused panID which has been found.
    #[must_use]
    pub const fn pan_id(&self) -> PanId {
        self.pan_id
    }

    /// The channel that the unused panID was found on.
    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }
}

impl Parameter for Handler {
    const ID: u16 = ID;
}
