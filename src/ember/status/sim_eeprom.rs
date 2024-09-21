use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::ember::status::values::Values;
use num_traits::FromPrimitive;

/// Ember simulated EEPROM status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SimEeprom {
    /// The Simulated EEPROM is telling the application that there is at least one flash page to be erased.
    ///
    /// The GREEN status means the current page has not filled above the `ERASE_CRITICAL_THRESHOLD`.
    /// The application should call the function `halSimEepromErasePage` when it can to erase a page.
    ErasePageGreen,
    /// The Simulated EEPROM is telling the application that there is at least one flash page to be erased.
    ///
    /// The RED status means the current page has filled above the `ERASE_CRITICAL_THRESHOLD`.
    /// Due to the shrinking availability of write space, there is a danger of data loss.
    /// The application must call the function `halSimEepromErasePage` as soon as possible to erase a page.
    ErasePageRed,
    /// The Simulated EEPROM has run out of room to write any new data and the data trying to be set
    /// has been lost.
    ///
    /// This error code is the result of ignoring the `SIM_EEPROM_ERASE_PAGE_RED` error code.
    /// The application must call the function `halSimEepromErasePage` to make room for any
    /// further calls to set a token.
    Full,
    /// Attempt 1 to initialize the Simulated EEPROM has failed.
    ///
    /// This failure means the information already stored in Flash (or a lack thereof), is fatally
    /// incompatible with the token information compiled into the code image being run.
    Init1Failed,
    /// Attempt 2 to initialize the Simulated EEPROM has failed.
    ///
    /// This failure means Attempt 1 failed, and the token system failed to properly reload default
    /// tokens and reset the Simulated EEPROM.
    Init2Failed,
    /// Attempt 3 to initialize the Simulated EEPROM has failed.
    ///
    /// This failure means one or both of the tokens `TOKEN_MFG_NVDATA_VERSION` or
    /// `TOKEN_STACK_NVDATA_VERSION` were incorrect.
    Init3Failed,
}

impl Display for SimEeprom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ErasePageGreen => write!(f, "erase page green"),
            Self::ErasePageRed => write!(f, "erase page red"),
            Self::Full => write!(f, "full"),
            Self::Init1Failed => write!(f, "init #1 failed"),
            Self::Init2Failed => write!(f, "init #2 failed"),
            Self::Init3Failed => write!(f, "init #3 failed"),
        }
    }
}

impl Error for SimEeprom {}

impl From<SimEeprom> for Values {
    fn from(sim_eeprom: SimEeprom) -> Self {
        match sim_eeprom {
            SimEeprom::ErasePageGreen => Self::SimEepromErasePageGreen,
            SimEeprom::ErasePageRed => Self::SimEepromErasePageRed,
            SimEeprom::Full => Self::SimEepromFull,
            SimEeprom::Init1Failed => Self::SimEepromInit1Failed,
            SimEeprom::Init2Failed => Self::SimEepromInit2Failed,
            SimEeprom::Init3Failed => Self::SimEepromInit3Failed,
        }
    }
}

impl From<SimEeprom> for u8 {
    fn from(sim_eeprom: SimEeprom) -> Self {
        Values::from(sim_eeprom).into()
    }
}

impl TryFrom<Values> for SimEeprom {
    type Error = Values;

    fn try_from(value: Values) -> Result<Self, Self::Error> {
        match value {
            Values::SimEepromErasePageGreen => Ok(Self::ErasePageGreen),
            Values::SimEepromErasePageRed => Ok(Self::ErasePageRed),
            Values::SimEepromFull => Ok(Self::Full),
            Values::SimEepromInit1Failed => Ok(Self::Init1Failed),
            Values::SimEepromInit2Failed => Ok(Self::Init2Failed),
            Values::SimEepromInit3Failed => Ok(Self::Init3Failed),
            _ => Err(value),
        }
    }
}

impl FromPrimitive for SimEeprom {
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
