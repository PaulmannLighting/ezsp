use crate::ember::key::Status;
use crate::ember::Eui64;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x009B;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    partner: Eui64,
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(partner: Eui64, status: Status) -> Self {
        Self {
            partner,
            status: status.into(),
        }
    }

    #[must_use]
    pub const fn partner(&self) -> Eui64 {
        self.partner
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
