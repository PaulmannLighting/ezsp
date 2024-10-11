use le_stream::derive::FromLeStream;

use crate::ember::Status;
use crate::frame;
use crate::frame::Parameter;

const ID: u16 = 0x0032;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    index: u8,
    policy_decision: u8,
}

impl Handler {
    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    pub fn policy_decision(&self) -> Result<Status, u8> {
        Status::try_from(self.policy_decision)
    }
}

impl Parameter<frame::Extended<frame::Response>> for Handler {
    const ID: u16 = ID;
}
