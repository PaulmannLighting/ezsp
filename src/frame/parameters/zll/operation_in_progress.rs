use crate::frame::Identified;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00D7;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    zll_operation_in_progress: bool,
}

impl Response {
    #[must_use]
    pub const fn zll_operation_in_progress(&self) -> bool {
        self.zll_operation_in_progress
    }
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
