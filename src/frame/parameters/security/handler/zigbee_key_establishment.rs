use le_stream::derive::FromLeStream;

use crate::ember::key::Status;
use crate::ember::Eui64;
use crate::frame::Parameter;

const ID: u16 = 0x009B;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    partner: Eui64,
    status: u8,
}

impl Handler {
    #[must_use]
    pub const fn partner(&self) -> Eui64 {
        self.partner
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
