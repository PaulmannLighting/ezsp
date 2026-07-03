//! Parameters for the  [`Security::import_transient_key`](crate::Security::import_transient_key) command.

use num_traits::FromPrimitive;
use silizium::Status;
use silizium::zigbee::security::man::{Context, Flags, Key};

use crate::Error;
use crate::ember::Eui64;

crate::frame::parameters::frame!(0x0111, { context: Context, eui64: Eui64, plaintext_key: Key, flags: u8 }, { status: u32 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(context: Context, eui64: Eui64, plaintext_key: Key, flags: Flags) -> Self {
        Self {
            context,
            eui64,
            plaintext_key,
            flags: flags.bits(),
        }
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
