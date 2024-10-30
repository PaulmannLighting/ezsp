//! Parameters for the [`Mfglib::get_power`](crate::Mfglib::get_power) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Identified;

const ID: u16 = 0x008D;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    power: i8,
}

impl Response {
    /// Returns the power level in dBm.
    #[must_use]
    pub const fn power(&self) -> i8 {
        self.power
    }
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
