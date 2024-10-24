use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::PanId;
use crate::frame::Identified;

const ID: u16 = 0x0057;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    new_pan: PanId,
}

impl Command {
    #[must_use]
    pub const fn new(new_pan: PanId) -> Self {
        Self { new_pan }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: bool,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl From<Response> for bool {
    fn from(response: Response) -> Self {
        response.status
    }
}
