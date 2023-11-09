use crate::types::ByteSizedVec;

use crate::ember::aes::MmoHashContext;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x006F;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    context: MmoHashContext,
    finalize: bool,
    data: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(context: MmoHashContext, finalize: bool, data: ByteSizedVec<u8>) -> Self {
        Self {
            context,
            finalize,
            data,
        }
    }

    #[must_use]
    pub const fn context(&self) -> &MmoHashContext {
        &self.context
    }

    #[must_use]
    pub const fn finalize(&self) -> bool {
        self.finalize
    }

    #[must_use]
    pub const fn length(&self) -> u8 {
        self.data.len() as u8
    }

    #[must_use]
    pub const fn data(&self) -> &ByteSizedVec<u8> {
        &self.data
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    return_context: MmoHashContext,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, return_context: MmoHashContext) -> Self {
        Self {
            status: status.into(),
            return_context,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn return_context(&self) -> &MmoHashContext {
        &self.return_context
    }
}
