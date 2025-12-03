use core::fmt::{self, Display};

/// Frame Format Version.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FormatVersion {
    /// Reserved bitflag.
    Reserved,
    /// Version 1.
    One,
    /// Version 0.
    Zero,
}

impl FormatVersion {
    /// Returns a string representation of the format version.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reserved => "Reserved",
            Self::One => "1",
            Self::Zero => "0",
        }
    }
}

impl Display for FormatVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
