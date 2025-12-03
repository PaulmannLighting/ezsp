//! Parameters for the [`Utilities::get_phy_interface_count`](crate::Utilities::get_phy_interface_count) command.

use le_stream::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00FC;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    interface_count: u8,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into the number of physical interfaces.
impl From<Response> for u8 {
    fn from(response: Response) -> Self {
        response.interface_count
    }
}
