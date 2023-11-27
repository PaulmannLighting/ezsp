use le_stream::derive::{FromLeBytes, ToLeBytes};

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Address {
    id: [u8; 8],
    application_id: u8,
    endpoint: u8,
}

impl Address {
    #[must_use]
    pub const fn new(id: [u8; 8], application_id: u8, endpoint: u8) -> Self {
        Self {
            id,
            application_id,
            endpoint,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &[u8; 8] {
        &self.id
    }

    #[must_use]
    pub const fn application_id(&self) -> u8 {
        self.application_id
    }

    #[must_use]
    pub const fn endpoint(&self) -> u8 {
        self.endpoint
    }
}
