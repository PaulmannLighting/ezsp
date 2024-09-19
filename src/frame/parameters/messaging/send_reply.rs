use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::aps::Frame;
use crate::ember::{NodeId, Status};
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x0039;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    sender: NodeId,
    aps_frame: Frame,
    message: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(sender: NodeId, aps_frame: Frame, message: ByteSizedVec<u8>) -> Self {
        Self {
            sender,
            aps_frame,
            message,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
