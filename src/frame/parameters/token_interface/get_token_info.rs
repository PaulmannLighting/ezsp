//! Parameters for the [`TokenInterface::get_token_info`](crate::TokenInterface::get_token_info) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::token::Info;

crate::frame::parameters::frame!(0x0101, { index: u8 }, { status: u8, token_info: Info });

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }
}

/// Convert the response into [`Info`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for Info {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.token_info),
            other => Err(other.into()),
        }
    }
}
