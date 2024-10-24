use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::key::Data;
use crate::ember::zll::InitialSecurityState;
use crate::ember::Status;
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x00B3;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    network_key: Data,
    security_state: InitialSecurityState,
}

impl Command {
    #[must_use]
    pub const fn new(network_key: Data, security_state: InitialSecurityState) -> Self {
        Self {
            network_key,
            security_state,
        }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
