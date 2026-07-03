//! Parameters for the [`Configuration::set_value`](crate::Configuration::set_value) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ezsp::Status;
use crate::ezsp::value::Id;
use crate::types::ByteSizedVec;

crate::frame::parameters::frame!(0x00AB, { value_id: u8, value: ByteSizedVec<u8> }, { status: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(value_id: Id, value: ByteSizedVec<u8>) -> Self {
        Self {
            value_id: value_id as u8,
            value,
        }
    }
}

/// Converts the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
