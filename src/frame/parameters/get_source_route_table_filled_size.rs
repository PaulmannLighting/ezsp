pub const ID: u16 = 0x00C2;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    source_route_table_filled_size: u8,
}

impl Response {
    #[must_use]
    pub const fn new(source_route_table_filled_size: u8) -> Self {
        Self {
            source_route_table_filled_size,
        }
    }

    #[must_use]
    pub const fn source_route_table_filled_size(&self) -> u8 {
        self.source_route_table_filled_size
    }
}
