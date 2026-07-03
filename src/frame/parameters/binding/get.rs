//! Parameters for the [`Binding::get_binding`](crate::Binding::get) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::binding::TableEntry;

crate::frame::parameters::frame!(0x002C, { index: u8 }, { status: u8, value: TableEntry });

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }
}

/// Convert the response into its [`TableEntry`] or an appropriate [`Error`]
/// by evaluating its status field.
impl TryFrom<Response> for TableEntry {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.value),
            other => Err(other.into()),
        }
    }
}
