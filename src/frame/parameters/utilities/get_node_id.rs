use crate::ember::NodeId;
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0027;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    node_id: NodeId,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl From<Response> for NodeId {
    fn from(response: Response) -> Self {
        response.node_id
    }
}
