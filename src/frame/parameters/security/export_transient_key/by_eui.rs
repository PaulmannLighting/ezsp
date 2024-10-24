use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use siliconlabs::Status;

use super::transient_key::TransientKey;
use crate::ember::Eui64;
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x0113;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    eui: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(eui: Eui64) -> Self {
        Self { eui }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    payload: TransientKey,
    status: u32,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Response> for TransientKey {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u32(response.status).ok_or(response.status) {
            Ok(Status::Ok) => Ok(response.payload),
            other => Err(other.into()),
        }
    }
}
