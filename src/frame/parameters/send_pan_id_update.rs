
pub const ID: u16 = 0x0057;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    new_pan: EmberPanId,
}

impl Command {
    #[must_use]
    pub const fn new(new_pan: EmberPanId) -> Self {
        Self { new_pan }
    }

    #[must_use]
    pub const fn new_pan(&self) -> EmberPanId {
        self.new_pan
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    status: bool,
}

impl Response {
    #[must_use]
    pub const fn new(status: bool) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> bool {
        self.status
    }
}
