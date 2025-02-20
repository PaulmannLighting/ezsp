//! Parameters for the [`Networking::get_network_parameters`](crate::Networking::get_network_parameters) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::network::Parameters;
use crate::ember::node::Type;
use crate::ember::Status;
use crate::error::ValueError;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0028;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    node_type: u8,
    parameters: Parameters,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert a response into a [`Type`] and [`Parameters`] tuple
/// or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for (Type, Parameters) {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Type::from_u8(response.node_type)
                .ok_or_else(|| ValueError::EmberNodeType(response.node_type).into())
                .map(|node_type| (node_type, response.parameters)),
            other => Err(other.into()),
        }
    }
}
