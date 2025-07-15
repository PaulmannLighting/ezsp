use std::error::Error;
use std::fmt::{Display, Formatter};

use num_traits::FromPrimitive;

use super::values::Values;

/// Ember application status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Application {
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error0,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error1,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error2,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error3,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error4,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error5,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error6,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error7,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error8,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error9,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error10,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error11,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error12,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error13,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error14,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error15,
}

impl Display for Application {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error0 => write!(f, "error #0"),
            Self::Error1 => write!(f, "error #1"),
            Self::Error2 => write!(f, "error #2"),
            Self::Error3 => write!(f, "error #3"),
            Self::Error4 => write!(f, "error #4"),
            Self::Error5 => write!(f, "error #5"),
            Self::Error6 => write!(f, "error #6"),
            Self::Error7 => write!(f, "error #7"),
            Self::Error8 => write!(f, "error #8"),
            Self::Error9 => write!(f, "error #9"),
            Self::Error10 => write!(f, "error #10"),
            Self::Error11 => write!(f, "error #11"),
            Self::Error12 => write!(f, "error #12"),
            Self::Error13 => write!(f, "error #13"),
            Self::Error14 => write!(f, "error #14"),
            Self::Error15 => write!(f, "error #15"),
        }
    }
}

impl Error for Application {}

impl From<Application> for Values {
    fn from(application: Application) -> Self {
        match application {
            Application::Error0 => Self::ApplicationError0,
            Application::Error1 => Self::ApplicationError1,
            Application::Error2 => Self::ApplicationError2,
            Application::Error3 => Self::ApplicationError3,
            Application::Error4 => Self::ApplicationError4,
            Application::Error5 => Self::ApplicationError5,
            Application::Error6 => Self::ApplicationError6,
            Application::Error7 => Self::ApplicationError7,
            Application::Error8 => Self::ApplicationError8,
            Application::Error9 => Self::ApplicationError9,
            Application::Error10 => Self::ApplicationError10,
            Application::Error11 => Self::ApplicationError11,
            Application::Error12 => Self::ApplicationError12,
            Application::Error13 => Self::ApplicationError13,
            Application::Error14 => Self::ApplicationError14,
            Application::Error15 => Self::ApplicationError15,
        }
    }
}

impl From<Application> for u8 {
    fn from(application: Application) -> Self {
        Values::from(application).into()
    }
}

impl TryFrom<Values> for Application {
    type Error = Values;

    fn try_from(value: Values) -> Result<Self, Self::Error> {
        match value {
            Values::ApplicationError0 => Ok(Self::Error0),
            Values::ApplicationError1 => Ok(Self::Error1),
            Values::ApplicationError2 => Ok(Self::Error2),
            Values::ApplicationError3 => Ok(Self::Error3),
            Values::ApplicationError4 => Ok(Self::Error4),
            Values::ApplicationError5 => Ok(Self::Error5),
            Values::ApplicationError6 => Ok(Self::Error6),
            Values::ApplicationError7 => Ok(Self::Error7),
            Values::ApplicationError8 => Ok(Self::Error8),
            Values::ApplicationError9 => Ok(Self::Error9),
            Values::ApplicationError10 => Ok(Self::Error10),
            Values::ApplicationError11 => Ok(Self::Error11),
            Values::ApplicationError12 => Ok(Self::Error12),
            Values::ApplicationError13 => Ok(Self::Error13),
            Values::ApplicationError14 => Ok(Self::Error14),
            Values::ApplicationError15 => Ok(Self::Error15),
            _ => Err(value),
        }
    }
}

impl FromPrimitive for Application {
    fn from_i64(n: i64) -> Option<Self> {
        Values::from_i64(n).and_then(|value| Self::try_from(value).ok())
    }

    fn from_u8(n: u8) -> Option<Self> {
        Values::from_u8(n).and_then(|value| Self::try_from(value).ok())
    }

    fn from_u64(n: u64) -> Option<Self> {
        Values::from_u64(n).and_then(|value| Self::try_from(value).ok())
    }
}
