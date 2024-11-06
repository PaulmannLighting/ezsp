//! Parameters for the [`Utilities::get_node_id`](crate::Utilities::get_node_id) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::NodeId;
use crate::frame::Parameter;

const ID: u16 = 0x0027;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    node_id: NodeId,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into the node ID.
impl From<Response> for NodeId {
    fn from(response: Response) -> Self {
        response.node_id
    }
}
