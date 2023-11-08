pub const ID: u16 = 0x00BC;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    data: EmberTokTypeStackZllData,
    security: EmberTokTypeStackZllSecurity,
}

impl Response {
    #[must_use]
    pub const fn new(
        data: EmberTokTypeStackZllData,
        security: EmberTokTypeStackZllSecurity,
    ) -> Self {
        Self { data, security }
    }

    #[must_use]
    pub const fn data(&self) -> EmberTokTypeStackZllData {
        self.data
    }

    #[must_use]
    pub const fn security(&self) -> EmberTokTypeStackZllSecurity {
        self.security
    }
}
