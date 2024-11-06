//! Parameters for the [`Utilities::get_eui64`](crate::Utilities::get_eui64) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::Eui64;
use crate::frame::Parameter;

const ID: u16 = 0x0026;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    eui64: Eui64,
}

impl Response {
    /// Returns the EUI64.
    #[must_use]
    pub const fn eui64(self) -> Eui64 {
        self.eui64
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Converts a [`Response`] into an [`Eui64`].
impl From<Response> for Eui64 {
    fn from(response: Response) -> Self {
        response.eui64
    }
}
