
pub const ID: u16 = 0x005B;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    address_table_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(address_table_index: u8) -> Self {
        Self { address_table_index }
    }

    #[must_use]
    pub const fn address_table_index(&self) -> u8 {
        self.address_table_index
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    active: bool,
}

impl Response {
    #[must_use]
    pub const fn new(active: bool) -> Self {
        Self { active }
    }

    #[must_use]
    pub const fn active(&self) -> bool {
        self.active
    }
}
