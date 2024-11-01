//! Parameters for the [`Configuration::send_pan_id_update`](crate::Configuration::send_pan_id_update) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::PanId;
use crate::frame::Identified;

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

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: bool,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Converts the response into a [`bool`] indicating whether the command was successful.
impl From<Response> for bool {
    fn from(response: Response) -> Self {
        response.status
    }
}
