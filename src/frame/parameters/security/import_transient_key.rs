use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use siliconlabs::zigbee::security::{ManContext, ManFlags, ManKey};
use siliconlabs::Status;

use crate::ember::Eui64;
use crate::frame::Parameter;
use crate::{Error, ValueError};

const ID: u16 = 0x0111;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
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
    const ID: Option<Self::Id> = Some(ID);
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u32,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        Status::from_u32(response.status)
            .ok_or_else(|| ValueError::Siliconlabs(response.status).into())
            .and_then(|status| {
                if status == Status::Ok {
                    Ok(())
                } else {
                    Err(status.into())
                }
            })
    }
}
