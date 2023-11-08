
pub const ID: u16 = 0x009E;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    status: EmberStatus,
    ephemeral_public_key: EmberPublicKeyData,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, ephemeral_public_key: EmberPublicKeyData) -> Self {
        Self { status, ephemeral_public_key }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }


    #[must_use]
    pub const fn ephemeral_public_key(&self) -> EmberPublicKeyData {
        self.ephemeral_public_key
    }
}
