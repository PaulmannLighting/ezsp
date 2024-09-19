use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x0008;

#[derive(Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    num_beacons: u8,
}

impl Response {
    #[must_use]
    pub const fn num_beacons(&self) -> u8 {
        self.num_beacons
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
