use crate::ember::Status;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0051;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    message: ByteSizedVec<u8>,
    priority: u8,
    use_cca: bool,
}

impl Command {
    #[must_use]
    pub const fn new(message_contents: ByteSizedVec<u8>, priority: u8, use_cca: bool) -> Self {
        Self {
            message: message_contents,
            priority,
            use_cca,
        }
    }

    #[must_use]
    pub const fn message(&self) -> &ByteSizedVec<u8> {
        &self.message
    }

    #[must_use]
    pub const fn priority(&self) -> u8 {
        self.priority
    }

    #[must_use]
    pub const fn use_cca(&self) -> bool {
        self.use_cca
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
