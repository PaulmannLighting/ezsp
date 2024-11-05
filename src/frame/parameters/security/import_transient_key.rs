//! Parameters for the  [`Security::import_transient_key`](crate::Security::import_transient_key) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use siliconlabs::zigbee::security::man::{Context, Flags, Key};
use siliconlabs::Status;

use crate::ember::Eui64;
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x0111;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    context: Context<Eui64>,
    eui64: Eui64,
    plaintext_key: Key,
    flags: u8,
}

impl Command {
    #[must_use]
    pub const fn new(
        context: Context<Eui64>,
        eui64: Eui64,
        plaintext_key: Key,
        flags: Flags,
    ) -> Self {
        Self {
            context,
            eui64,
            plaintext_key,
            flags: flags.bits(),
        }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u32,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u32(response.status).ok_or(response.status) {
            Ok(Status::Ok) => Ok(()),
            other => Err(other.into()),
        }
    }
}
