mod control;

pub use control::Control;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const HEADER_SIZE: usize = 5;
pub const LEGACY_HEADER_SIZE: usize = 3;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Header {
    sequence: u8,
    control: Control,
    id: u16,
}

impl Header {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, id: u16) -> Self {
        Self {
            sequence,
            control,
            id,
        }
    }

    #[must_use]
    pub const fn for_frame<const ID: u16>(sequence: u8, control: Control) -> Self {
        Self::new(sequence, control, ID)
    }

    #[must_use]
    pub const fn control(&self) -> &Control {
        &self.control
    }

    #[must_use]
    pub const fn id(&self) -> u16 {
        self.id
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Eq, PartialEq)]
pub struct LegacyHeader {
    sequence: u8,
    control: u8,
    id: u8,
}

impl LegacyHeader {
    #[must_use]
    pub const fn new(sequence: u8, control: u8, id: u8) -> Self {
        Self {
            sequence,
            control,
            id,
        }
    }

    #[must_use]
    pub const fn for_frame<const ID: u8>(sequence: u8, control: u8) -> Self {
        Self::new(sequence, control, ID)
    }

    #[must_use]
    pub const fn control(&self) -> u8 {
        self.control
    }

    #[must_use]
    pub const fn id(&self) -> u8 {
        self.id
    }
}
