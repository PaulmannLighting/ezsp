
pub const ID: u16 = 0x004A;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    status: EmberStatus,
    child_data: EmberChildData,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, child_data: EmberChildData) -> Self {
        Self { status, child_data }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }


    #[must_use]
    pub const fn child_data(&self) -> EmberChildData {
        self.child_data
    }
}
