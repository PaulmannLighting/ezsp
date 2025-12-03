use core::fmt::{self, Display};

/// Sleep mode states.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SleepMode {
    /// Reserved.
    Reserved,
    /// Power down.
    PowerDown,
    /// Deep sleep.
    DeepSleep,
    /// Idle.
    Idle,
}

impl SleepMode {
    /// Returns a string representation of the sleep mode.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reserved => "Reserved",
            Self::PowerDown => "Power Down",
            Self::DeepSleep => "Deep Sleep",
            Self::Idle => "Idle",
        }
    }
}

impl Display for SleepMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
