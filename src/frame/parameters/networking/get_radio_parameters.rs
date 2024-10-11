use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::multi_phy::radio::Parameters;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x00FD;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    phy_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(phy_index: u8) -> Self {
        Self { phy_index }
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    parameters: Parameters,
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = Parameters;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.parameters)
    }
}
