use core::fmt::Display;

const MAJOR: u16 = 0b1111_0000_0000_0000;
const MINOR: u16 = 0b0000_1111_0000_0000;
const PATCH: u16 = 0b0000_0000_1111_0000;
const RELEASE: u16 = 0b0000_0000_0000_1111;

/// `EZSP` stack version.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct StackVersion(pub u16);

impl StackVersion {
    /// Returns the major version.
    #[must_use]
    pub const fn major(self) -> u8 {
        ((self.0 & MAJOR) >> 12) as u8
    }

    /// Returns the minor version.
    #[must_use]
    pub const fn minor(self) -> u8 {
        ((self.0 & MINOR) >> 8) as u8
    }

    /// Returns the patch version.
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub const fn patch(self) -> u8 {
        ((self.0 & PATCH) >> 4) as u8
    }

    /// Returns the release version.
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub const fn release(self) -> u8 {
        (self.0 & RELEASE) as u8
    }
}

impl Display for StackVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch(),)?;

        if self.release() != 0 {
            write!(f, "-{}", self.release())?;
        }

        Ok(())
    }
}

impl From<u16> for StackVersion {
    fn from(version: u16) -> Self {
        Self(version)
    }
}
