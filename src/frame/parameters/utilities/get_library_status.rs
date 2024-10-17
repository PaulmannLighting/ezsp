use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::library::{Id, Status};
use crate::frame::Parameter;

const ID: u16 = 0x0001;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    library_id: Id,
}

impl Command {
    #[must_use]
    pub const fn new(library_id: Id) -> Self {
        Self { library_id }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: Status,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl From<Response> for Status {
    fn from(response: Response) -> Self {
        response.status
    }
}
