//! Parameters for the [`Security::import_key`](crate::Security::import_key) command.

use num_traits::FromPrimitive;
use silizium::Status;
use silizium::zigbee::security::man::{Context, Key};

use crate::Error;

crate::frame::parameters::frame!(0x0115, { context: Context, key: Key }, { status: u32 });

impl Command {
    #[must_use]
    pub const fn new(context: Context, key: Key) -> Self {
        Self { context, key }
    }
}

/// Convert the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u32(response.status).ok_or(response.status) {
            Ok(Status::Ok) => Ok(()),
            other => Err(other.into()),
        }
    }
}
