#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SleepMode {
    Reserved,
    PowerDown,
    DeepSleep,
    Idle,
}
