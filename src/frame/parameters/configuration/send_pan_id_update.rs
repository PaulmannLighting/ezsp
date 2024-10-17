use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::PanId;
use crate::frame::Parameter;

const ID: u16 = 0x0057;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    new_pan: PanId,
}

impl Command {
    #[must_use]
    pub const fn new(new_pan: PanId) -> Self {
        Self { new_pan }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: bool,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

impl From<Response> for bool {
    fn from(response: Response) -> Self {
        response.status
    }
}
