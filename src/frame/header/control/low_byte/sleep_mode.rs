use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(Debug, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum SleepMode {
    Reserved = 0b11,
    PowerDown = 0b01,
    SeepSleep = 0b10,
    Idle = 0b00,
}

impl From<SleepMode> for u8 {
    fn from(sleep_mode: SleepMode) -> Self {
        sleep_mode
            .to_u8()
            .expect("SleepMode should always be convertible to u8.")
    }
}
