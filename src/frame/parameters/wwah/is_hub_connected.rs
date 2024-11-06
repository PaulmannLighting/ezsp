//! Parameters for the [`Wwah::is_hub_connected`](crate::Wwah::is_hub_connected) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00E6;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    is_hub_connected: bool,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into a boolean indicating if the hub is connected.
impl From<Response> for bool {
    fn from(response: Response) -> Self {
        response.is_hub_connected
    }
}
