use crate::types::{ByteSizedVec, EmberStatus};

use crate::ember::aes::MmoHashContext;
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
    status: EmberStatus,
    return_context: MmoHashContext,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, return_context: MmoHashContext) -> Self {
        Self {
            status,
            return_context,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn return_context(&self) -> &MmoHashContext {
        &self.return_context
    }
}
