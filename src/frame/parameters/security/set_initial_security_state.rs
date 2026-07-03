//! Parameters for the [`Security::set_initial_security_state`](crate::Security::set_initial_security_state) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::security::initial::State;

crate::frame::parameters::frame!(0x0068, { state: State }, { success: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(state: State) -> Self {
        Self { state }
    }
}

/// Convert the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.success).ok_or(response.success) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
