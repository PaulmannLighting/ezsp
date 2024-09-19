use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::{Eui64, Status};
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x005C;

#[derive(Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    address_table_index: u8,
    eui64: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(address_table_index: u8, eui64: Eui64) -> Self {
        Self {
            address_table_index,
            eui64,
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

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
