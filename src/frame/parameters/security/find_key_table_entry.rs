//! Parameters for the [`Security::find_key_table_entry`](crate::Security::find_key_table_entry) command.

use crate::ember::Eui64;

crate::frame::parameters::frame!(0x0075, { address: Eui64, link_key: bool }, { index: u8 });

impl Command {
    #[must_use]
    pub const fn new(address: Eui64, link_key: bool) -> Self {
        Self { address, link_key }
    }
}

/// Convert the response into the index of the key table entry.
impl From<Response> for u8 {
    fn from(response: Response) -> Self {
        response.index
    }
}
