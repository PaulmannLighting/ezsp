//! Parameters for the [`Wwah::get_parent_classification_enabled`](crate::Wwah::get_parent_classification_enabled) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Identified;

const ID: u16 = 0x00F0;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    enabled: bool,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert the response into a boolean indicating if the parent classification is enabled.
impl From<Response> for bool {
    fn from(response: Response) -> Self {
        response.enabled
    }
}
