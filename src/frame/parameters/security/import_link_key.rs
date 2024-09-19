use crate::ember::Eui64;
use crate::error::Resolve;
use crate::frame::Parameter;
use crate::Error;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use siliconlabs::zigbee::security::ManKey;
use siliconlabs::Status;

const ID: u16 = 0x010E;

#[derive(Clone, Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    index: u8,
    address: Eui64,
    plaintext_key: ManKey,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8, address: Eui64, plaintext_key: ManKey) -> Self {
        Self {
            index,
            address,
            plaintext_key,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u32,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = ();

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve()
    }
}
