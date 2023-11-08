
pub const ID: u16 = 0x0071;

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
    key_struct: EmberKeyStruct,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, key_struct: EmberKeyStruct) -> Self {
        Self { status, key_struct }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }


    #[must_use]
    pub const fn key_struct(&self) -> EmberKeyStruct {
        self.key_struct
    }
}
