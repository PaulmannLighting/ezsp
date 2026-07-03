//! Parameters for the [`Security::request_link_key`](crate::Security::request_link_key) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{Eui64, Status};

crate::frame::parameters::frame!(0x0014, { partner: Eui64 }, { status: u8 });

impl Command {
    #[must_use]
    pub const fn new(partner: Eui64) -> Self {
        Self { partner }
    }
}

/// Convert the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
