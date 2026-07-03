//! Parameters for the [`Cbke::clear_temporary_data_maybe_store_link_key`](crate::Cbke::clear_temporary_data_maybe_store_link_key) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::frame!(0x00EE, { store_link_key: bool }, { status: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(store_link_key: bool) -> Self {
        Self { store_link_key }
    }
}

/// Converts the response into `()` or an appropriate [`Error`] by evaluating its status field.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
