//! Parameters for the [`Wwah::get_parent_classification_enabled`](crate::Wwah::get_parent_classification_enabled) command.

use le_stream::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00F0;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    enabled: bool,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into a boolean indicating if the parent classification is enabled.
impl From<Response> for bool {
    fn from(response: Response) -> Self {
        response.enabled
    }
}
