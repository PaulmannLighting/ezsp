use core::fmt::{self, Display};

/// Callback types.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CallbackType {
    /// Reserved.
    Reserved,
    /// Async callback actively sent by the NCP.
    Async,
    /// Sync callback actively requested by the host.
    Sync,
}

impl CallbackType {
    /// Returns a string representation of the callback type.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reserved => "Reserved",
            Self::Async => "Async",
            Self::Sync => "Sync",
        }
    }
}

impl Display for CallbackType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
