use crate::ember::library::{Id, Status};
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0001;

#[derive(Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Command {
    library_id: Id,
}

impl Command {
    #[must_use]
    pub const fn new(library_id: Id) -> Self {
        Self { library_id }
    }

    #[must_use]
    pub const fn library_id(&self) -> Id {
        self.library_id
    }
}

#[derive(Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Response {
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }
}
