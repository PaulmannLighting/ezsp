//! Parameters for the [`Networking::network_init`](crate::Networking::network_init) command.

use itertools::Itertools;
use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::ezsp::network::InitBitmask;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0017;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    bitmask: u16,
}

impl Command {
    #[must_use]
    pub fn new(bitmask: &[InitBitmask]) -> Self {
        Self {
            bitmask: bitmask.iter().unique().copied().map(u16::from).sum(),
        }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert a response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
