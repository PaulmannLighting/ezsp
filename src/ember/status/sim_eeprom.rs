use std::error::Error;
use std::fmt::{Display, Formatter};

use num_derive::FromPrimitive;

/// Ember simulated EEPROM status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum SimEeprom {
    /// The Simulated EEPROM is telling the application that there is at least one flash page to be erased.
    ///
    /// The GREEN status means the current page has not filled above the `ERASE_CRITICAL_THRESHOLD`.
    /// The application should call the function `halSimEepromErasePage` when it can to erase a page.
    ErasePageGreen = 0x43,
    /// The Simulated EEPROM is telling the application that there is at least one flash page to be erased.
    ///
    /// The RED status means the current page has filled above the `ERASE_CRITICAL_THRESHOLD`.
    /// Due to the shrinking availability of write space, there is a danger of data loss.
    /// The application must call the function `halSimEepromErasePage` as soon as possible to erase a page.
    ErasePageRed = 0x44,
    /// The Simulated EEPROM has run out of room to write any new data and the data trying to be set
    /// has been lost.
    ///
    /// This error code is the result of ignoring the `SIM_EEPROM_ERASE_PAGE_RED` error code.
    /// The application must call the function `halSimEepromErasePage` to make room for any
    /// further calls to set a token.
    Full = 0x45,
    /// Attempt 1 to initialize the Simulated EEPROM has failed.
    ///
    /// This failure means the information already stored in Flash (or a lack thereof), is fatally
    /// incompatible with the token information compiled into the code image being run.
    Init1Failed = 0x48,
    /// Attempt 2 to initialize the Simulated EEPROM has failed.
    ///
    /// This failure means Attempt 1 failed, and the token system failed to properly reload default
    /// tokens and reset the Simulated EEPROM.
    Init2Failed = 0x49,
    /// Attempt 3 to initialize the Simulated EEPROM has failed.
    ///
    /// This failure means one or both of the tokens `TOKEN_MFG_NVDATA_VERSION` or
    /// `TOKEN_STACK_NVDATA_VERSION` were incorrect.
    Init3Failed = 0x4A,
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

impl From<SimEeprom> for u8 {
    fn from(sim_eeprom: SimEeprom) -> Self {
        sim_eeprom as Self
    }
}

impl Error for SimEeprom {}
