//! Parameters for the [`Networking::join_network`](crate::Networking::join_network) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::network::Parameters;
use crate::ember::node::Type;

crate::frame::parameters::frame!(0x001F, { node_type: u8, parameters: Parameters }, { status: u8 });

impl Command {
    #[must_use]
    pub fn new(node_type: Type, parameters: Parameters) -> Self {
        Self {
            node_type: node_type.into(),
            parameters,
        }
    }
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
