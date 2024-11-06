//! Parameters for the [`Zll::set_node_type`](crate::Zll::set_node_type) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::node::Type;
use crate::frame::Parameter;

const ID: u16 = 0x00D5;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    node_type: u8,
}

impl Command {
    #[must_use]
    pub fn new(node_type: Type) -> Self {
        Self {
            node_type: node_type.into(),
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
