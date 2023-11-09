use crate::types::{EmberNetworkParameters, EmberNodeType, EmberStatus};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0028;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: EmberStatus,
    node_type: EmberNodeType,
    parameters: EmberNetworkParameters,
}

impl Response {
    #[must_use]
    pub const fn new(
        status: EmberStatus,
        node_type: EmberNodeType,
        parameters: EmberNetworkParameters,
    ) -> Self {
        Self {
            status,
            node_type,
            parameters,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn node_type(&self) -> EmberNodeType {
        self.node_type
    }

    #[must_use]
    pub const fn parameters(&self) -> EmberNetworkParameters {
        self.parameters
    }
}
