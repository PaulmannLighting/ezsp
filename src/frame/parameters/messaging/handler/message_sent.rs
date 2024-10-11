use le_stream::derive::FromLeStream;

use crate::ember::aps::Frame;
use crate::ember::message::Outgoing;
use crate::ember::Status;
use crate::frame;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x003F;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    typ: u8,
    index_or_destination: u16,
    aps_frame: Frame,
    message_tag: u8,
    status: u8,
    message: ByteSizedVec<u8>,
}

impl Handler {
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

impl Parameter<frame::Extended<frame::Response>> for Handler {
    const ID: u16 = ID;
}
