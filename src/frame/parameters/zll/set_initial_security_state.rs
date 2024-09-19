use crate::ember::key::Data;
use crate::ember::zll::InitialSecurityState;
use crate::ember::Status;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00B3;

#[derive(Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
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

    #[must_use]
    pub const fn network_key(&self) -> Data {
        self.network_key
    }

    #[must_use]
    pub const fn security_state(&self) -> &InitialSecurityState {
        &self.security_state
    }
}

#[derive(Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
