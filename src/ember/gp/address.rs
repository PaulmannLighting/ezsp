use le_stream::derive::{FromLeBytes, ToLeBytes};

/// A GP address structure.
#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Address {
    id: [u8; 8],
    application_id: u8,
    endpoint: u8,
}

impl Address {
    /// Create a new GP address.
    #[must_use]
    pub const fn new(id: [u8; 8], application_id: u8, endpoint: u8) -> Self {
        Self {
            id,
            application_id,
            endpoint,
        }
    }

    /// Return what either contains a 4-byte source ID or an 8-byte IEEE address,
    /// as indicated by the value of the applicationId field.
    #[must_use]
    pub const fn id(&self) -> &[u8; 8] {
        &self.id
    }

    /// Return the GPD Application ID specifying either source ID (`0x00`) or IEEE address (`0x02`).
    #[must_use]
    pub const fn application_id(&self) -> u8 {
        self.application_id
    }

    /// Return the GPD endpoint.
    #[must_use]
    pub const fn endpoint(&self) -> u8 {
        self.endpoint
    }
}
