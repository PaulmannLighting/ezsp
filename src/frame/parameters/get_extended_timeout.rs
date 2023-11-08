pub const ID: u16 = 0x007F;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    remote_eui64: EmberEUI64,
}

impl Command {
    #[must_use]
    pub const fn new(remote_eui64: EmberEUI64) -> Self {
        Self { remote_eui64 }
    }

    #[must_use]
    pub const fn remote_eui64(&self) -> EmberEUI64 {
        self.remote_eui64
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    extended_timeout: bool,
}

impl Response {
    #[must_use]
    pub const fn new(extended_timeout: bool) -> Self {
        Self { extended_timeout }
    }

    #[must_use]
    pub const fn extended_timeout(&self) -> bool {
        self.extended_timeout
    }
}
