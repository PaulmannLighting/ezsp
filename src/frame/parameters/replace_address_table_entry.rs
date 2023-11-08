pub const ID: u16 = 0x0082;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    address_table_index: u8,
    new_eui64: EmberEUI64,
    new_id: EmberNodeId,
    new_extended_timeout: bool,
}

impl Command {
    #[must_use]
    pub const fn new(
        address_table_index: u8,
        new_eui64: EmberEUI64,
        new_id: EmberNodeId,
        new_extended_timeout: bool,
    ) -> Self {
        Self {
            address_table_index,
            new_eui64,
            new_id,
            new_extended_timeout,
        }
    }

    #[must_use]
    pub const fn address_table_index(&self) -> u8 {
        self.address_table_index
    }

    #[must_use]
    pub const fn new_eui64(&self) -> EmberEUI64 {
        self.new_eui64
    }

    #[must_use]
    pub const fn new_id(&self) -> EmberNodeId {
        self.new_id
    }

    #[must_use]
    pub const fn new_extended_timeout(&self) -> bool {
        self.new_extended_timeout
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EmberStatus,
    old_eui64: EmberEUI64,
    old_id: EmberNodeId,
    old_extended_timeout: bool,
}

impl Response {
    #[must_use]
    pub const fn new(
        status: EmberStatus,
        old_eui64: EmberEUI64,
        old_id: EmberNodeId,
        old_extended_timeout: bool,
    ) -> Self {
        Self {
            status,
            old_eui64,
            old_id,
            old_extended_timeout,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn old_eui64(&self) -> EmberEUI64 {
        self.old_eui64
    }

    #[must_use]
    pub const fn old_id(&self) -> EmberNodeId {
        self.old_id
    }

    #[must_use]
    pub const fn old_extended_timeout(&self) -> bool {
        self.old_extended_timeout
    }
}
