use std::num::TryFromIntError;

const OFF: u8 = 0x00;
const ON: u8 = 0xFF;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Duration {
    Disable,
    Seconds(Seconds),
    Enable,
}

impl From<u8> for Duration {
    fn from(value: u8) -> Self {
        match value {
            OFF => Self::Disable,
            0x01..=0xFE => Self::Seconds(Seconds(value)),
            ON => Self::Enable,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<std::time::Duration> for Duration {
    type Error = TryFromIntError;

    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        u8::try_from(value.as_secs()).map(Self::from)
    }
}

impl From<Duration> for u8 {
    fn from(value: Duration) -> Self {
        match value {
            Duration::Disable => 0x00,
            Duration::Seconds(value) => value.into(),
            Duration::Enable => 0xFF,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Seconds(u8);

impl From<Seconds> for u8 {
    fn from(value: Seconds) -> Self {
        value.0
    }
}
