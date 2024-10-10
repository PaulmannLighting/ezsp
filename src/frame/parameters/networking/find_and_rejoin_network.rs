use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x0021;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    have_current_network_key: bool,
    channel_mask: u32,
}

impl Command {
    #[must_use]
    pub const fn new(have_current_network_key: bool, channel_mask: u32) -> Self {
        Self {
            have_current_network_key,
            channel_mask,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
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
