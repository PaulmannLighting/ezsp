pub const ID: u16 = 0x00C7;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EmberStatus,
    gpep_handle: u8,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, gpep_handle: u8) -> Self {
        Self {
            status,
            gpep_handle,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn gpep_handle(&self) -> u8 {
        self.gpep_handle
    }
}
