pub const ID: u16 = 0x009B;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    partner: EmberEUI64,
    status: EmberKeyStatus,
}

impl Response {
    #[must_use]
    pub const fn new(partner: EmberEUI64, status: EmberKeyStatus) -> Self {
        Self { partner, status }
    }

    #[must_use]
    pub const fn partner(&self) -> EmberEUI64 {
        self.partner
    }

    #[must_use]
    pub const fn status(&self) -> EmberKeyStatus {
        self.status
    }
}
