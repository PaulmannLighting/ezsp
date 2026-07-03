//! Parameters for the [`Bootloader::launch_standalone_bootloader()`](crate::Bootloader::launch_standalone_bootloader) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::frame!(0x008F, { mode: u8 }, { status: u8 });

impl Command {
    #[must_use]
    pub const fn new(mode: u8) -> Self {
        Self { mode }
    }
}

/// Convert the response into a [`Result<()>`](crate::Result) by evaluating its status field.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
