use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00D7;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Response {
    zll_operation_in_progress: bool,
}

impl Response {
    #[must_use]
    pub const fn new(zll_operation_in_progress: bool) -> Self {
        Self {
            zll_operation_in_progress,
        }
    }

    #[must_use]
    pub const fn zll_operation_in_progress(&self) -> bool {
        self.zll_operation_in_progress
    }
}
