use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{};

pub const ID: u16 = 0x005A;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    mode: u8,
}

impl Command {
    #[must_use]
    pub const fn new(mode: u8) -> Self {
        Self { mode }
    }

    #[must_use]
    pub const fn mode(&self) -> u8 {
        self.mode
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    remaining_time: u32,
}

impl Response {
    #[must_use]
    pub const fn new(remaining_time: u32) -> Self {
        Self { remaining_time }
    }

    #[must_use]
    pub const fn remaining_time(&self) -> u32 {
        self.remaining_time
    }
}
