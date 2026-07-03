//! Parameters for the [`TokenInterface::set_token_data`](crate::TokenInterface::set_token_data) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::token::Data;

crate::frame::parameters::frame!(0x0103, { token: u32, index: u32, token_data: Data }, { status: u8 });

impl Command {
    #[must_use]
    pub const fn new(token: u32, index: u32, token_data: Data) -> Self {
        Self {
            token,
            index,
            token_data,
        }
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
