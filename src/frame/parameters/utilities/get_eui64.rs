//! Parameters for the [`Utilities::get_eui64`](crate::Utilities::get_eui64) command.

use crate::ember::Eui64;

crate::frame::parameters::frame!(0x0026, {}, { eui64: Eui64 });

impl Response {
    /// Returns the EUI64.
    #[must_use]
    pub const fn eui64(self) -> Eui64 {
        self.eui64
    }
}

/// Converts a [`Response`] into an [`Eui64`].
impl From<Response> for Eui64 {
    fn from(response: Response) -> Self {
        response.eui64
    }
}
