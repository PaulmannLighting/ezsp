use crate::ember::Eui64;
use le_stream::derive::{FromLeBytes, ToLeBytes};

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct DeviceInfoRecord {
    ieee_address: Eui64,
    endpoint_id: u8,
    profile_id: u16,
    device_id: u16,
    version: u8,
    group_id_count: u8,
}

impl DeviceInfoRecord {
    #[must_use]
    pub const fn new(
        ieee_address: Eui64,
        endpoint_id: u8,
        profile_id: u16,
        device_id: u16,
        version: u8,
        group_id_count: u8,
    ) -> Self {
        Self {
            ieee_address,
            endpoint_id,
            profile_id,
            device_id,
            version,
            group_id_count,
        }
    }

    #[must_use]
    pub fn ieee_address(&self) -> Eui64 {
        self.ieee_address
    }

    #[must_use]
    pub fn endpoint_id(&self) -> u8 {
        self.endpoint_id
    }

    #[must_use]
    pub fn profile_id(&self) -> u16 {
        self.profile_id
    }

    #[must_use]
    pub fn device_id(&self) -> u16 {
        self.device_id
    }

    #[must_use]
    pub fn version(&self) -> u8 {
        self.version
    }

    #[must_use]
    pub fn group_id_count(&self) -> u8 {
        self.group_id_count
    }
}
