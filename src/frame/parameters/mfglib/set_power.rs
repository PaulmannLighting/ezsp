use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x008C;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    tx_power_mode: u16,
    power: i8,
}

impl Command {
    #[must_use]
    pub const fn new(tx_power_mode: u16, power: i8) -> Self {
        Self {
            tx_power_mode,
            power,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = ();

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve()
    }
}
