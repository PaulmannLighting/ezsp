use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::{Eui64, Status};
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x0090;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    broadcast: bool,
    dest_eui64: Eui64,
    message: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(broadcast: bool, dest_eui64: Eui64, message: ByteSizedVec<u8>) -> Self {
        Self {
            broadcast,
            dest_eui64,
            message,
        }
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
