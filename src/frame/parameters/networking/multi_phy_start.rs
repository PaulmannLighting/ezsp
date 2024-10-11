use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::multi_phy::nwk::Config;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Resolve;
use crate::{frame, Error};

const ID: u16 = 0x00F8;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    phy_index: u8,
    page: u8,
    channel: u8,
    power: i8,
    bitmask: u8,
}

impl Command {
    #[must_use]
    pub fn new(phy_index: u8, page: u8, channel: u8, power: i8, bitmask: Config) -> Self {
        Self {
            phy_index,
            page,
            channel,
            power,
            bitmask: bitmask.into(),
        }
    }
}

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
