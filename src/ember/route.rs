use le_stream::derive::{FromLeStream, ToLeStream};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

const ENTRY_UNUSED: u16 = 0xFFFF;

/// Ember route table entry status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Status {
    /// Entry is active.
    Active = 0x00,
    /// Entry is being discovered.
    Discovered = 0x01,
    /// Entry is unused.
    Unused = 0x03,
    /// Entry is validating.
    Validating = 0x04,
}

impl From<Status> for u8 {
    fn from(status: Status) -> Self {
        status as Self
    }
}

impl TryFrom<u8> for Status {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

/// Ember route table entry concentrator type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum ConcentratorType {
    /// Destination is not a concentrator.
    NotAConcentrator = 0x00,
    /// Destination is a Low RAM Concentrator.
    LowRam = 0x01,
    /// Destination is a High RAM Concentrator.
    HighRam = 0x02,
}

impl From<ConcentratorType> for u8 {
    fn from(concentrator_type: ConcentratorType) -> Self {
        concentrator_type as Self
    }
}

impl TryFrom<u8> for ConcentratorType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

/// Ember route table entry route record state.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum RecordState {
    /// Route record is no longer needed.
    NoLongerNeeded = 0x00,
    /// A route record has been sent.
    Sent = 0x01,
    /// A route record is needed.
    Needed = 0x02,
}

impl From<RecordState> for u8 {
    fn from(route_record_state: RecordState) -> Self {
        route_record_state as Self
    }
}

impl TryFrom<u8> for RecordState {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

/// A route table entry stores information about the next hop along the route to the destination.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct TableEntry {
    destination: u16,
    next_hop: u16,
    status: u8,
    age: u8,
    concentrator_type: u8,
    route_record_state: u8,
}

impl TableEntry {
    /// Create a new route table entry.
    #[must_use]
    pub fn new(
        destination: u16,
        next_hop: u16,
        status: Status,
        age: u8,
        concentrator_type: ConcentratorType,
        route_record_state: RecordState,
    ) -> Self {
        Self {
            destination,
            next_hop,
            status: status.into(),
            age,
            concentrator_type: concentrator_type.into(),
            route_record_state: route_record_state.into(),
        }
    }

    /// Return the short id of the destination.
    ///
    /// A value of `0xFFFF` is translated to `None` and indicates the entry is unused.
    #[must_use]
    pub const fn destination(&self) -> Option<u16> {
        if self.destination == ENTRY_UNUSED {
            None
        } else {
            Some(self.destination)
        }
    }

    /// Return the short id of the next hop to this destination.
    #[must_use]
    pub const fn next_hop(&self) -> u16 {
        self.next_hop
    }

    /// Return the status of the entry.
    ///
    /// # Errors
    /// Returns the invalid status code number if the status is invalid.
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    /// Return the number of seconds since this route entry was last used to send a packet.
    #[must_use]
    pub const fn age(&self) -> u8 {
        self.age
    }

    /// Return the concentrator type of the entry.
    ///
    /// # Errors
    /// Returns the invalid concentrator type code number if the concentrator type is invalid.
    pub fn concentrator_type(&self) -> Result<ConcentratorType, u8> {
        ConcentratorType::try_from(self.concentrator_type)
    }

    /// Return the route record state of the entry.
    ///
    /// # Errors
    /// Returns the invalid route record state code number if the route record state is invalid.
    pub fn route_record_state(&self) -> Result<RecordState, u8> {
        if self.concentrator_type() == Ok(ConcentratorType::HighRam) {
            RecordState::try_from(self.route_record_state)
        } else {
            Err(self.route_record_state)
        }
    }
}
