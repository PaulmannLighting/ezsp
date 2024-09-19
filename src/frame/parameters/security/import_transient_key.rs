use crate::ember::Eui64;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use siliconlabs::zigbee::security::{ManContext, ManFlags, ManKey};
use siliconlabs::Status;

const ID: u16 = 0x0111;

#[derive(Clone, Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    context: ManContext,
    eui64: Eui64,
    plaintext_key: ManKey,
    flags: u8,
}

impl Command {
    #[must_use]
    pub fn new(context: ManContext, eui64: Eui64, plaintext_key: ManKey, flags: ManFlags) -> Self {
        Self {
            context,
            eui64,
            plaintext_key,
            flags: flags.into(),
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
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
