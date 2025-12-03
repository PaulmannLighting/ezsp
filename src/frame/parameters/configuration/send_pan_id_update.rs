//! Parameters for the [`Configuration::send_pan_id_update`](crate::Configuration::send_pan_id_update) command.

use le_stream::{FromLeStream, ToLeStream};

use crate::ember::PanId;
use crate::frame::Parameter;

const ID: u16 = 0x0057;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    new_pan: PanId,
}

impl Command {
    #[must_use]
    pub const fn new(new_pan: PanId) -> Self {
        Self { new_pan }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: bool,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Converts the response into a [`bool`] indicating whether the command was successful.
impl From<Response> for bool {
    fn from(response: Response) -> Self {
        response.status
    }
}
