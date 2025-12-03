//! Parameters for the [`Wwah::is_uptime_long`](crate::Wwah::is_uptime_long) command.

use le_stream::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00E5;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    has_long_up_time: bool,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into a boolean value of whether the uptime is long.
impl From<Response> for bool {
    fn from(response: Response) -> Self {
        response.has_long_up_time
    }
}
