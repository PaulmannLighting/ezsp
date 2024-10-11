use crate::frame::Parameter;
use crate::Resolve;
use crate::{frame, Error};
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0077;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    exclude_outgoing_fc: bool,
    exclude_boot_counter: bool,
}

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
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

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Ok(())
    }
}
