//! Parameters for the [`ProxyTable::get_entry`](crate::ProxyTable::get_entry) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::gp::proxy::TableEntry;

crate::frame::parameters::frame!(0x00C8, { proxy_index: u8 }, { status: u8, entry: TableEntry });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(proxy_index: u8) -> Self {
        Self { proxy_index }
    }
}

/// Converts the response into a [`TableEntry`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for TableEntry {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.entry),
            other => Err(other.into()),
        }
    }
}
