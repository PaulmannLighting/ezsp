use crate::ember::gp::security::FrameCounter;
use crate::ember::gp::sink::{ListEntry, LIST_ENTRIES};
use crate::ember::gp::Address;
use crate::ember::key::Data;
use crate::ember::NodeId;
use le_stream::derive::{FromLeBytes, ToLeBytes};

type Status = u8;

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct TableEntry {
    status: Status,
    options: u32,
    gpd: Address,
    assigned_alias: NodeId,
    security_options: u8,
    gpd_security_frame_counter: FrameCounter,
    gpd_key: Data,
    sink_list: [ListEntry; LIST_ENTRIES],
    group_cast_radius: u8,
    search_counter: u8,
}

impl TableEntry {
    #[must_use]
    pub const fn new(
        status: Status,
        options: u32,
        gpd: Address,
        assigned_alias: NodeId,
        security_options: u8,
        gpd_security_frame_counter: FrameCounter,
        gpd_key: Data,
        sink_list: [ListEntry; LIST_ENTRIES],
        group_cast_radius: u8,
        search_counter: u8,
    ) -> Self {
        Self {
            status,
            options,
            gpd,
            assigned_alias,
            security_options,
            gpd_security_frame_counter,
            gpd_key,
            sink_list,
            group_cast_radius,
            search_counter,
        }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    #[must_use]
    pub const fn options(&self) -> u32 {
        self.options
    }

    #[must_use]
    pub const fn gpd(&self) -> &Address {
        &self.gpd
    }

    #[must_use]
    pub const fn assigned_alias(&self) -> NodeId {
        self.assigned_alias
    }

    #[must_use]
    pub const fn security_options(&self) -> u8 {
        self.security_options
    }

    #[must_use]
    pub const fn gpd_security_frame_counter(&self) -> FrameCounter {
        self.gpd_security_frame_counter
    }

    #[must_use]
    pub const fn gpd_key(&self) -> &Data {
        &self.gpd_key
    }

    #[must_use]
    pub const fn sink_list(&self) -> &[ListEntry; LIST_ENTRIES] {
        &self.sink_list
    }

    #[must_use]
    pub const fn group_cast_radius(&self) -> u8 {
        self.group_cast_radius
    }

    #[must_use]
    pub const fn search_counter(&self) -> u8 {
        self.search_counter
    }
}
