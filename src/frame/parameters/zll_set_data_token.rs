use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberTokTypeStackZllData};

pub const ID: u16 = 0x00BD;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    data: EmberTokTypeStackZllData,
}

impl Command {
    #[must_use]
    pub const fn new(data: EmberTokTypeStackZllData) -> Self {
        Self { data }
    }

    #[must_use]
    pub const fn data(&self) -> EmberTokTypeStackZllData {
        self.data
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response;

impl Response {
    #[must_use]
    pub const fn new() -> Self {
        Self {  }
    }
}
