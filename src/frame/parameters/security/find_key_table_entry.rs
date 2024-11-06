//! Parameters for the [`Security::find_key_table_entry`](crate::Security::find_key_table_entry) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::Eui64;
use crate::frame::Parameter;

const ID: u16 = 0x0075;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    address: Eui64,
    link_key: bool,
}

impl Command {
    #[must_use]
    pub const fn new(address: Eui64, link_key: bool) -> Self {
        Self { address, link_key }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    index: u8,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into the index of the key table entry.
impl From<Response> for u8 {
    fn from(response: Response) -> Self {
        response.index
    }
}
