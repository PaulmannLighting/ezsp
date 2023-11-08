
pub const ID: u16 = 0x001F;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    node_type: EmberNodeType,
    parameters: EmberNetworkParameters,
}

impl Command {
    #[must_use]
    pub const fn new(node_type: EmberNodeType, parameters: EmberNetworkParameters) -> Self {
        Self { node_type, parameters }
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

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
