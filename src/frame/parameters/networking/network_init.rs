use itertools::Itertools;
use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::Status;
use crate::ezsp::network::InitBitmask;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x0017;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    bitmask: u16,
}

impl Command {
    #[must_use]
    pub fn new(bitmask: &[InitBitmask]) -> Self {
        Self {
            bitmask: bitmask.iter().unique().copied().map(u16::from).sum(),
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
