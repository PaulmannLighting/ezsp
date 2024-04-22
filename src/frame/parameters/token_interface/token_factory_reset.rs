use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0077;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    exclude_outgoing_fc: bool,
    exclude_boot_counter: bool,
}

impl Command {
    #[must_use]
    pub const fn new(exclude_outgoing_fc: bool, exclude_boot_counter: bool) -> Self {
        Self {
            exclude_outgoing_fc,
            exclude_boot_counter,
        }
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response;
