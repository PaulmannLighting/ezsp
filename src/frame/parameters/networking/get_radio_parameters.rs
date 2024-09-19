use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::multi_phy::radio::Parameters;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x00FD;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    phy_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(phy_index: u8) -> Self {
        Self { phy_index }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    parameters: Parameters,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = Parameters;

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.parameters)
    }
}
