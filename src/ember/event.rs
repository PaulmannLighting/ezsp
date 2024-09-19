use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Ember event units.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Units {
    /// The event is not scheduled to run.
    Inactive = 0x00,
    /// The execution time is in approximate milliseconds.
    MsTime = 0x01,
    /// The execution time is in 'binary' quarter seconds (256 approximate milliseconds each).
    QsTime = 0x02,
    /// The execution time is in 'binary' minutes (65536 approximate milliseconds each).
    MinuteTime = 0x03,
}

impl From<Units> for u8 {
    fn from(units: Units) -> Self {
        units as Self
    }
}

impl TryFrom<u8> for Units {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
