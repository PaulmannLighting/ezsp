
pub const ID: u16 = 0x00E1;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    addr: EmberGpAddress,
}

impl Command {
    #[must_use]
    pub const fn new(addr: EmberGpAddress) -> Self {
        Self { addr }
    }

    #[must_use]
    pub const fn addr(&self) -> EmberGpAddress {
        self.addr
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    index: u8,
}

impl Response {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }
}
