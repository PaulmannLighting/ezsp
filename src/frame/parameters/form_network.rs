use crate::ember::network::Parameters;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x001E;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    parameters: Parameters,
}

impl Command {
    #[must_use]
    pub const fn new(parameters: Parameters) -> Self {
        Self { parameters }
    }

    #[must_use]
    pub const fn parameters(&self) -> &Parameters {
        &self.parameters
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
