use crate::ember::aps::Frame;
use crate::ember::message::Outgoing;
use crate::ember::Status;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x003F;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    typ: u8,
    index_or_destination: u16,
    aps_frame: Frame,
    message_tag: u8,
    status: u8,
    message: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub fn new(
        typ: Outgoing,
        index_or_destination: u16,
        aps_frame: Frame,
        message_tag: u8,
        status: Status,
        message: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            typ: typ.into(),
            index_or_destination,
            aps_frame,
            message_tag,
            status: status.into(),
            message,
        }
    }

    pub fn typ(&self) -> Result<Outgoing, u8> {
        Outgoing::try_from(self.typ)
    }

    #[must_use]
    pub const fn index_or_destination(&self) -> u16 {
        self.index_or_destination
    }

    #[must_use]
    pub const fn aps_frame(&self) -> &Frame {
        &self.aps_frame
    }

    #[must_use]
    pub const fn message_tag(&self) -> u8 {
        self.message_tag
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn message(&self) -> &ByteSizedVec<u8> {
        &self.message
    }
}
