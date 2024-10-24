use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Identified;

const ID: u16 = 0x0008;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Identified for Command {
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

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
