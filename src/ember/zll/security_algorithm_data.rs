use le_stream::derive::{FromLeBytes, ToLeBytes};

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct SecurityAlgorithmData {
    transaction_id: u32,
    response_id: u32,
    bitmask: u16,
}

impl SecurityAlgorithmData {
    #[must_use]
    pub const fn new(transaction_id: u32, response_id: u32, bitmask: u16) -> Self {
        Self {
            transaction_id,
            response_id,
            bitmask,
        }
    }

    #[must_use]
    pub const fn transaction_id(&self) -> u32 {
        self.transaction_id
    }

    #[must_use]
    pub const fn response_id(&self) -> u32 {
        self.response_id
    }

    #[must_use]
    pub const fn bitmask(&self) -> u16 {
        self.bitmask
    }
}
