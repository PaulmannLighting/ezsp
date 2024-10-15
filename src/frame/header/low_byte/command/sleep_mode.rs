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
