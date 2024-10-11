use le_stream::derive::FromLeStream;

use crate::ember::Status;
use crate::frame;
use crate::frame::Parameter;

const ID: u16 = 0x00C7;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    gpep_handle: u8,
}

impl Handler {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn gpep_handle(&self) -> u8 {
        self.gpep_handle
    }
}

impl Parameter<frame::Extended<frame::Response>> for Handler {
    const ID: u16 = ID;
}
