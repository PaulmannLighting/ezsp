//! Parameters for the [`Utilities::get_library_status`](crate::Utilities::get_library_status) command.

use le_stream::{FromLeStream, ToLeStream};

use crate::ember::library::{Id, Status};
use crate::frame::Parameter;

const ID: u16 = 0x0001;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    library_id: Id,
}

impl Command {
    #[must_use]
    pub const fn new(library_id: Id) -> Self {
        Self { library_id }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: Status,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

impl From<Response> for Status {
    fn from(response: Response) -> Self {
        response.status
    }
}
