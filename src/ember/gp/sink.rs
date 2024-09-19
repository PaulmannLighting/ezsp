/// Ember GP sink structs.
///
/// For details, see https://docs.silabs.com/d/zigbee-stack-api/7.2.2/gp-types-h
use le_stream::derive::{FromLeStream, ToLeStream};
use le_stream::FromLeStream;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::ember::gp::security::FrameCounter;
use crate::ember::key::Data;
use crate::ember::{gp, Eui64, NodeId};

/// Amount of entries in a sink list.
pub const LIST_ENTRIES: usize = 2;

/// Type alias for the sink status.
pub type Status = u8;

/// Ember GP sink address.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Address {
    sink_eui: Eui64,
    sink_node_id: NodeId,
}

impl Address {
    /// Create a new GP sink address.
    #[must_use]
    pub const fn new(sink_eui: Eui64, sink_node_id: NodeId) -> Self {
        Self {
            sink_eui,
            sink_node_id,
        }
    }

    /// Return the EUI64 of the sink.
    #[must_use]
    pub const fn sink_eui(&self) -> Eui64 {
        self.sink_eui
    }

    /// Return the node ID of the sink.
    #[must_use]
    pub const fn sink_node_id(&self) -> NodeId {
        self.sink_node_id
    }
}

/// Ember GP sink group.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Group {
    group_id: u16,
    alias: u16,
}

impl Group {
    /// Create a new GP sink group.
    #[must_use]
    pub const fn new(group_id: u16, alias: u16) -> Self {
        Self { group_id, alias }
    }

    /// Return the group ID.
    #[must_use]
    pub const fn group_id(&self) -> u16 {
        self.group_id
    }

    /// Return the alias.
    #[must_use]
    pub const fn alias(&self) -> u16 {
        self.alias
    }
}

/// Enver GP sink type.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Type {
    /// Full unicast.
    FullUnicast = 0x00,
    /// D-group cast.
    DGroupCast = 0x01,
    /// Group cast.
    GroupCast = 0x02,
    /// LW-unicast.
    LwUnicast = 0x03,
    /// Unused.
    Unused = 0xFF,
}

impl From<Type> for u8 {
    fn from(typ: Type) -> Self {
        typ as Self
    }
}

impl TryFrom<u8> for Type {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

/// Ember GP sink payload.
#[derive(Debug, Eq, PartialEq)]
pub enum Payload {
    /// Unicast.
    Unicast(Address),
    /// Group cast.
    GroupCast(Group),
    /// Group list.
    GroupList(Group),
}

/// Ember GP sink list entry.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct ListEntry {
    typ: u8,
    bytes: [u8; 10], // Size of union
}

impl ListEntry {
    /// Returns the type of the list entry.
    ///
    /// # Errors
    /// Returns the [`u8`] value of the type if it is not a valid [`Type`].
    pub fn typ(&self) -> Result<Type, u8> {
        Type::try_from(self.typ)
    }

    /// Returns the payload of the list entry.
    ///
    /// # Returns
    /// [`Some(Payload)`] if it is a valid type.
    ///
    /// # Errors
    /// Returns `None` if the type is [`Type::Unused`].
    #[must_use]
    pub fn payload(&self) -> Option<Payload> {
        self.typ().map_or(None, |typ| match typ {
            Type::DGroupCast | Type::GroupCast => {
                Group::from_le_stream(&mut self.bytes.iter().copied())
                    .ok()
                    .map(Payload::GroupList)
            }
            Type::LwUnicast | Type::FullUnicast => {
                Address::from_le_stream(&mut self.bytes.iter().copied())
                    .ok()
                    .map(Payload::Unicast)
            }
            Type::Unused => None,
        })
    }
}

/// The internal representation of a sink table entry.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct TableEntry {
    status: Status,
    options: u32,
    gpd: gp::Address,
    device_id: u8,
    sink_list: [ListEntry; LIST_ENTRIES],
    assigned_alias: NodeId,
    group_cast_radius: u8,
    security_options: u8,
    gpd_security_frame_counter: FrameCounter,
    gpd_key: Data,
}

impl TableEntry {
    /// Create a new sink table entry.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        status: Status,
        options: u32,
        gpd: gp::Address,
        device_id: u8,
        sink_list: [ListEntry; LIST_ENTRIES],
        assigned_alias: NodeId,
        group_cast_radius: u8,
        security_options: u8,
        gpd_security_frame_counter: FrameCounter,
        gpd_key: Data,
    ) -> Self {
        Self {
            status,
            options,
            gpd,
            device_id,
            sink_list,
            assigned_alias,
            group_cast_radius,
            security_options,
            gpd_security_frame_counter,
            gpd_key,
        }
    }

    /// Return the internal status of the sink table entry.
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
    pub const fn gpd(&self) -> &gp::Address {
        &self.gpd
    }

    /// Return the device id for the GPD.
    #[must_use]
    pub const fn device_id(&self) -> u8 {
        self.device_id
    }

    /// Return the list of sinks (hardcoded to 2 which is the spec minimum).
    #[must_use]
    pub const fn sink_list(&self) -> &[ListEntry; LIST_ENTRIES] {
        &self.sink_list
    }

    /// Return the assigned alias for the GPD.
    #[must_use]
    pub const fn assigned_alias(&self) -> NodeId {
        self.assigned_alias
    }

    /// Return the assigned alias for the GPD.
    #[must_use]
    pub const fn group_cast_radius(&self) -> u8 {
        self.group_cast_radius
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
}
