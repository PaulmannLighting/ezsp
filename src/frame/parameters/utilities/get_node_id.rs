//! Parameters for the [`Utilities::get_node_id`](crate::Utilities::get_node_id) command.

use crate::ember::NodeId;

crate::frame::parameters::frame!(0x0027, {}, { node_id: NodeId });

/// Convert the response into the node ID.
impl From<Response> for NodeId {
    fn from(response: Response) -> Self {
        response.node_id
    }
}
