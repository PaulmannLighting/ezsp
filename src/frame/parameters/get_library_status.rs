use crate::types::{EmberLibraryId, EmberLibraryStatus};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0001;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    library_id: EmberLibraryId,
}

impl Command {
    #[must_use]
    pub const fn new(library_id: EmberLibraryId) -> Self {
        Self { library_id }
    }

    #[must_use]
    pub const fn library_id(&self) -> EmberLibraryId {
        self.library_id
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: EmberLibraryStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberLibraryStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberLibraryStatus {
        self.status
    }
}
