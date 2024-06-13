use num_derive::FromPrimitive;

#[derive(Debug, Clone, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum SleepMode {
    Reserved = 0b11,
    PowerDown = 0b01,
    SeepSleep = 0b10,
    Idle = 0b00,
}

impl From<SleepMode> for u8 {
    fn from(sleep_mode: SleepMode) -> Self {
        sleep_mode as Self
    }
}
