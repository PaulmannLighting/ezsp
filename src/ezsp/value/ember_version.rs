use std::array::TryFromSliceError;
use std::fmt::Display;

use crate::types::ByteSizedVec;

/// Ember version information.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct EmberVersion {
    build: u16,
    major: u8,
    minor: u8,
    patch: u8,
    special: u8,
    typ: u8,
}

impl EmberVersion {
    /// Returns the build number.
    #[must_use]
    pub const fn build(self) -> u16 {
        self.build
    }

    /// Returns the major version.
    #[must_use]
    pub const fn major(self) -> u8 {
        self.major
    }

    /// Returns the minor version.
    #[must_use]
    pub const fn minor(self) -> u8 {
        self.minor
    }

    /// Returns the patch version.
    #[must_use]
    pub const fn patch(self) -> u8 {
        self.patch
    }

    /// Returns the special version.
    #[must_use]
    pub const fn special(self) -> u8 {
        self.special
    }

    /// Returns the type of version.
    #[must_use]
    pub const fn typ(self) -> u8 {
        self.typ
    }
}

impl Display for EmberVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}-{} ({} / {})",
            self.major(),
            self.minor(),
            self.patch(),
            self.build(),
            self.typ(),
            self.special(),
        )
    }
}

impl From<[u8; 7]> for EmberVersion {
    fn from(bytes: [u8; 7]) -> Self {
        Self {
            build: u16::from_le_bytes([bytes[0], bytes[1]]),
            major: bytes[2],
            minor: bytes[3],
            patch: bytes[4],
            special: bytes[5],
            typ: bytes[6],
        }
    }
}

impl TryFrom<&[u8]> for EmberVersion {
    type Error = TryFromSliceError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        <[u8; 7]>::try_from(bytes).map(Self::from)
    }
}

impl TryFrom<ByteSizedVec<u8>> for EmberVersion {
    type Error = TryFromSliceError;

    fn try_from(value: ByteSizedVec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_slice())
    }
}
