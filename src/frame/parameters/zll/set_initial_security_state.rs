use crate::ember::key::Data;
use crate::ember::zll::InitialSecurityState;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::resolve::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00B3;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
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

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> crate::Result<Self::Output> {
        Status::try_from(self.status).resolve()
    }
}
