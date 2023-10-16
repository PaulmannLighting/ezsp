mod control;

pub use control::Control;
use std::io::Read;

pub const HEADER_SIZE: usize = 5;
pub const LEGACY_HEADER_SIZE: usize = 3;

#[derive(Debug, Eq, PartialEq)]
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

    /// Reads the header from a reader
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on read errors.
    pub fn read_from<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: Read,
    {
        let mut buffer = [0; HEADER_SIZE];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from(buffer))
    }
}

impl From<[u8; HEADER_SIZE]> for Header {
    fn from(bytes: [u8; HEADER_SIZE]) -> Self {
        let [sequence, control_low, control_high, id_0, id_1] = bytes;
        Self::new(
            sequence,
            Control::new(control_low, control_high),
            u16::from_be_bytes([id_0, id_1]),
        )
    }
}

impl From<&Header> for [u8; HEADER_SIZE] {
    fn from(header: &Header) -> Self {
        let [id_0, id_1] = header.id.to_be_bytes();
        [
            header.sequence,
            header.control.low(),
            header.control.high(),
            id_0,
            id_1,
        ]
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

    /// Reads the header from a reader
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on read errors.
    pub fn read_from<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: Read,
    {
        let mut buffer = [0; LEGACY_HEADER_SIZE];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from(buffer))
    }
}

impl From<[u8; LEGACY_HEADER_SIZE]> for LegacyHeader {
    fn from(bytes: [u8; LEGACY_HEADER_SIZE]) -> Self {
        let [sequence, control, id] = bytes;
        Self::new(sequence, control, id)
    }
}

impl From<&LegacyHeader> for [u8; LEGACY_HEADER_SIZE] {
    fn from(header: &LegacyHeader) -> Self {
        [header.sequence, header.control, header.id]
    }
}
