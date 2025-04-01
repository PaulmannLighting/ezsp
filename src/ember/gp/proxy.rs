//! Proxy table entry.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::NodeId;
use crate::ember::gp::Address;
use crate::ember::gp::security::FrameCounter;
use crate::ember::gp::sink::{LIST_ENTRIES, ListEntry};
use crate::ember::key::Data;

type Status = u8;

/// The internal representation of a proxy table entry.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
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
    /// Create a new proxy table entry.
    #[allow(clippy::too_many_arguments)]
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

    /// Return the internal status of the proxy table entry.
    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    /// Return the tunneling options (this contains both options and extendedOptions from the spec).
    #[must_use]
    pub const fn options(&self) -> u32 {
        self.options
    }

    /// Return the addressing info of the GPD.
    #[must_use]
    pub const fn gpd(&self) -> &Address {
        &self.gpd
    }

    /// Return the assigned alias for the GPD.
    #[must_use]
    pub const fn assigned_alias(&self) -> NodeId {
        self.assigned_alias
    }

    /// Return the security options field.
    #[must_use]
    pub const fn security_options(&self) -> u8 {
        self.security_options
    }

    /// Return the security frame counter of the GPD.
    #[must_use]
    pub const fn gpd_security_frame_counter(&self) -> FrameCounter {
        self.gpd_security_frame_counter
    }

    /// Return the key to use for GPD.
    #[must_use]
    pub const fn gpd_key(&self) -> &Data {
        &self.gpd_key
    }

    /// Return the list of sinks (hardcoded to 2 which is the spec minimum).
    #[must_use]
    pub const fn sink_list(&self) -> &[ListEntry; LIST_ENTRIES] {
        &self.sink_list
    }

    /// Return the group cast radius.
    #[must_use]
    pub const fn group_cast_radius(&self) -> u8 {
        self.group_cast_radius
    }

    /// Return the search counter.
    #[must_use]
    pub const fn search_counter(&self) -> u8 {
        self.search_counter
    }
}
