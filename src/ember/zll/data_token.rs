use le_stream::derive::{FromLeBytes, ToLeBytes};

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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

    #[must_use]
    pub const fn bitmask(&self) -> u32 {
        self.bitmask
    }

    #[must_use]
    pub const fn free_node_id_min(&self) -> u16 {
        self.free_node_id_min
    }

    #[must_use]
    pub const fn free_node_id_max(&self) -> u16 {
        self.free_node_id_max
    }

    #[must_use]
    pub const fn my_group_id_min(&self) -> u16 {
        self.my_group_id_min
    }

    #[must_use]
    pub const fn free_group_id_min(&self) -> u16 {
        self.free_group_id_min
    }

    #[must_use]
    pub const fn free_group_id_max(&self) -> u16 {
        self.free_group_id_max
    }

    #[must_use]
    pub const fn rssi_correction(&self) -> u8 {
        self.rssi_correction
    }
}
