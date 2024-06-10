use std::num::TryFromIntError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Duration {
    Disable,
    Seconds(u8),
    Enable,
}

impl From<u8> for Duration {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Disable,
            0x01..=0xFE => Self::Seconds(value),
            0xFF => Self::Enable,
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
            Duration::Seconds(value) => value,
            Duration::Enable => 0xFF,
        }
    }
}
