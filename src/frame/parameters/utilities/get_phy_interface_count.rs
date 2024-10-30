//! Parameters for the [`Utilities::get_phy_interface_count`](crate::Utilities::get_phy_interface_count) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Identified;

const ID: u16 = 0x00FC;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    interface_count: u8,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert the response into the number of physical interfaces.
impl From<Response> for u8 {
    fn from(response: Response) -> Self {
        response.interface_count
    }
}
