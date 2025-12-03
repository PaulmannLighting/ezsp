use le_stream::{FromLeStream, ToLeStream};

use crate::ember::Eui64;

/// Information about a specific ZLL Device.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct DeviceInfoRecord {
    ieee_address: Eui64,
    endpoint_id: u8,
    profile_id: u16,
    device_id: u16,
    version: u8,
    group_id_count: u8,
}

impl DeviceInfoRecord {
    /// Create a new ZLL Device Information Record.
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

    /// Return the EUI64 associated with the device.
    #[must_use]
    pub const fn ieee_address(&self) -> Eui64 {
        self.ieee_address
    }

    /// Return the endpoint ID.
    #[must_use]
    pub const fn endpoint_id(&self) -> u8 {
        self.endpoint_id
    }

    /// Return the profile ID.
    #[must_use]
    pub const fn profile_id(&self) -> u16 {
        self.profile_id
    }

    /// Return the device ID.
    #[must_use]
    pub const fn device_id(&self) -> u16 {
        self.device_id
    }

    /// Return the associated version.
    #[must_use]
    pub const fn version(&self) -> u8 {
        self.version
    }

    /// Return the number of relevant group IDs.
    #[must_use]
    pub const fn group_id_count(&self) -> u8 {
        self.group_id_count
    }
}
