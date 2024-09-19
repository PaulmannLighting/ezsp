use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0077;

#[derive(Clone, Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    exclude_outgoing_fc: bool,
    exclude_boot_counter: bool,
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
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

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response;

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Ok(())
    }
}
