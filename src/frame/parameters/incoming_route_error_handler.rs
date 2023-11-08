
pub const ID: u16 = 0x0080;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    status: EmberStatus,
    target: EmberNodeId,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, target: EmberNodeId) -> Self {
        Self { status, target }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }


    #[must_use]
    pub const fn target(&self) -> EmberNodeId {
        self.target
    }
}
