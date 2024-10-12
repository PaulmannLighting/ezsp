use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

const MAX_TIME: u16 = 32767;
const MILLIS_PER_BINARY_QUARTER_SEC: u128 = 256;
const MILLIS_PER_BINARY_MINUTE: u128 = 65536;

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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Duration {
    time: u16,
    units: Units,
}

impl Duration {
    /// Create a new duration.
    pub const fn try_new(time: u16, units: Units) -> Result<Self, u16> {
        if time > MAX_TIME {
            Err(time)
        } else {
            Ok(Self { time, units })
        }
    }

    /// Return the time.
    #[must_use]
    pub const fn time(&self) -> u16 {
        self.time
    }

    /// Return the units.
    #[must_use]
    pub const fn units(&self) -> Units {
        self.units
    }
}

impl TryFrom<Option<std::time::Duration>> for Duration {
    type Error = std::time::Duration;

    fn try_from(duration: Option<std::time::Duration>) -> Result<Self, Self::Error> {
        let Some(duration) = duration else {
            return Ok(Self {
                time: 0,
                units: Units::Inactive,
            });
        };

        let millis = duration.as_millis();

        if millis < MAX_TIME as u128 {
            return Ok(Self {
                time: millis as u16,
                units: Units::MsTime,
            });
        }

        let binary_quarter_secs = millis / MILLIS_PER_BINARY_QUARTER_SEC;

        if binary_quarter_secs < MAX_TIME as u128 {
            return Ok(Self {
                time: binary_quarter_secs as u16,
                units: Units::QsTime,
            });
        }

        let binary_minutes = millis / MILLIS_PER_BINARY_MINUTE;

        if binary_minutes < MAX_TIME as u128 {
            return Ok(Self {
                time: binary_minutes as u16,
                units: Units::MinuteTime,
            });
        }

        Err(duration)
    }
}
