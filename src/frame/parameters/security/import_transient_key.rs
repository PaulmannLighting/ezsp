use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use siliconlabs::zigbee::security::{ManContext, ManFlags, ManKey};
use siliconlabs::Status;

use crate::ember::Eui64;
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x0111;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    context: ManContext<Eui64>,
    eui64: Eui64,
    plaintext_key: ManKey,
    flags: u8,
}

impl Command {
    #[must_use]
    pub const fn new(
        context: ManContext<Eui64>,
        eui64: Eui64,
        plaintext_key: ManKey,
        flags: ManFlags,
    ) -> Self {
        Self {
            context,
            eui64,
            plaintext_key,
            flags: flags as u8,
        }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u32,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u32(response.status).ok_or(response.status) {
            Ok(Status::Ok) => Ok(()),
            other => Err(other.into()),
        }
    }
}
