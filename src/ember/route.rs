use le_stream::derive::{FromLeBytes, ToLeBytes};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

const ENTRY_UNUSED: u16 = 0xFFFF;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Status {
    Active = 0x00,
    Discovered = 0x01,
    Unused = 0x03,
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum ConcentratorType {
    NotAConcentrator = 0x00,
    LowRam = 0x01,
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum RecordState {
    NoLongerNeeded = 0x00,
    Sent = 0x01,
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

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct TableEntry {
    destination: u16,
    next_hop: u16,
    status: u8,
    age: u8,
    concentrator_type: u8,
    route_record_state: u8,
}

impl TableEntry {
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

    #[must_use]
    pub const fn destination(&self) -> Option<u16> {
        if self.destination == ENTRY_UNUSED {
            None
        } else {
            Some(self.destination)
        }
    }

    #[must_use]
    pub const fn next_hop(&self) -> u16 {
        self.next_hop
    }

    /// Returns the status of the entry.
    ///
    /// # Errors
    /// Returns the invalid status code number if the status is invalid.
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn age(&self) -> u8 {
        self.age
    }

    /// Returns the concentrator type of the entry.
    ///
    /// # Errors
    /// Returns the invalid concentrator type code number if the concentrator type is invalid.
    pub fn concentrator_type(&self) -> Result<ConcentratorType, u8> {
        ConcentratorType::try_from(self.concentrator_type)
    }

    /// Returns the route record state of the entry.
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
