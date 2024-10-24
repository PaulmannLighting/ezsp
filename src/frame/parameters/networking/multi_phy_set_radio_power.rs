use crate::ember::Status;
use crate::frame::Identified;
use crate::Error;

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

const ID: u16 = 0x00FA;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    phy_index: u8,
    power: i8,
}

impl Command {
    #[must_use]
    pub const fn new(phy_index: u8, power: i8) -> Self {
        Self { phy_index, power }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
