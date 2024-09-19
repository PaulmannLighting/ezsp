use le_stream::derive::{FromLeStream, ToLeStream};

/// Public API for ZLL stack data token.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct DataToken {
    bitmask: u32,
    free_node_id_min: u16,
    free_node_id_max: u16,
    my_group_id_min: u16,
    free_group_id_min: u16,
    free_group_id_max: u16,
    rssi_correction: u8,
}

impl DataToken {
    /// Create a new ZLL stack data token.
    #[must_use]
    pub const fn new(
        bitmask: u32,
        free_node_id_min: u16,
        free_node_id_max: u16,
        my_group_id_min: u16,
        free_group_id_min: u16,
        free_group_id_max: u16,
        rssi_correction: u8,
    ) -> Self {
        Self {
            bitmask,
            free_node_id_min,
            free_node_id_max,
            my_group_id_min,
            free_group_id_min,
            free_group_id_max,
            rssi_correction,
        }
    }

    /// Return the token bitmask.
    #[must_use]
    pub const fn bitmask(&self) -> u32 {
        self.bitmask
    }

    /// Return the minimum free node id.
    #[must_use]
    pub const fn free_node_id_min(&self) -> u16 {
        self.free_node_id_min
    }

    /// Return the maximum free node id.
    #[must_use]
    pub const fn free_node_id_max(&self) -> u16 {
        self.free_node_id_max
    }

    /// Return the local minimum group id.
    #[must_use]
    pub const fn my_group_id_min(&self) -> u16 {
        self.my_group_id_min
    }

    /// Return the minimum free group id.
    #[must_use]
    pub const fn free_group_id_min(&self) -> u16 {
        self.free_group_id_min
    }

    /// Return the maximum free group id.
    #[must_use]
    pub const fn free_group_id_max(&self) -> u16 {
        self.free_group_id_max
    }

    /// Return the RSSI correction value.
    #[must_use]
    pub const fn rssi_correction(&self) -> u8 {
        self.rssi_correction
    }
}
