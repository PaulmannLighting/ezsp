pub const ID: u16 = 0x0058;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    reason: EzspStatus,
}

impl Response {
    #[must_use]
    pub const fn new(reason: EzspStatus) -> Self {
        Self { reason }
    }

    #[must_use]
    pub const fn reason(&self) -> EzspStatus {
        self.reason
    }
}
