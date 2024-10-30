//! Parameters for the [`Networking::network_state`](crate::Networking::network_state) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::network::Status;
use crate::frame::Identified;
use crate::{Error, ValueError};

const ID: u16 = 0x0018;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert a response into a [`Status`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for Status {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        Self::from_u8(response.status)
            .ok_or_else(|| ValueError::EmberNetworkStatus(response.status).into())
    }
}
