use crate::frame::Identified;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0118;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    // The number of active entries in the sink table.
    // The documentation specifies `Uint_t`, but the actual type is `u8`.
    number_of_entries: u8,
}

impl Response {
    #[must_use]
    pub const fn number_of_entries(&self) -> u8 {
        self.number_of_entries
    }
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
