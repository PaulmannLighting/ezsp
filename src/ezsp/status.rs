use num_traits::FromPrimitive;

pub use ash::Ash;
pub use error::Error;
pub use spi_err::SpiErr;

use crate::Resolve;

mod ash;
mod error;
mod spi_err;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Status {
    Success = 0x00,
    SpiErr(SpiErr),
    AshInProgress = 0x20,
    HostFatalError = 0x21,
    AshNcpFatalError = 0x22,
    DataFrameTooLong = 0x23,
    DataFrameTooShort = 0x24,
    NoTxSpace = 0x25,
    NoRxSpace = 0x26,
    NoRxData = 0x27,
    NotConnected = 0x28,
    Error(Error),
    Ash(Ash),
    CpcErrorInit = 0x86,
    NoError = 0xFF,
}

impl Status {
    /// Checks the status for success and returns `Ok(value)` in that case.
    ///
    /// # Errors
    /// Returns `Err(self)` if the `Status` is not [`Status::Success`],
    pub fn map<T>(self, value: T) -> Result<T, Self> {
        if self == Self::Success {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// Checks the status for success and returns `Ok(())` in that case.
    ///
    /// # Errors
    /// Returns `Err(self)` if the `Status` is not [`Status::Success`],
    pub fn ok(self) -> Result<(), Self> {
        self.map(())
    }
}

impl From<Status> for u8 {
    fn from(status: Status) -> Self {
        match status {
            Status::Success => 0x00,
            Status::SpiErr(spi_err) => spi_err.into(),
            Status::AshInProgress => 0x20,
            Status::HostFatalError => 0x21,
            Status::AshNcpFatalError => 0x22,
            Status::DataFrameTooLong => 0x23,
            Status::DataFrameTooShort => 0x24,
            Status::NoTxSpace => 0x25,
            Status::NoRxSpace => 0x26,
            Status::NoRxData => 0x27,
            Status::NotConnected => 0x28,
            Status::Error(error) => error.into(),
            Status::Ash(ash) => ash.into(),
            Status::CpcErrorInit => 0x86,
            Status::NoError => 0xFF,
        }
    }
}

impl FromPrimitive for Status {
    fn from_i64(n: i64) -> Option<Self> {
        u8::try_from(n).ok().and_then(Self::from_u8)
    }

    fn from_u8(n: u8) -> Option<Self> {
        match n {
            0x00 => Some(Self::Success),
            0x10..=0x1D => SpiErr::from_u8(n).map(Self::SpiErr),
            0x20 => Some(Self::AshInProgress),
            0x21 => Some(Self::HostFatalError),
            0x22 => Some(Self::AshNcpFatalError),
            0x23 => Some(Self::DataFrameTooLong),
            0x24 => Some(Self::DataFrameTooShort),
            0x25 => Some(Self::NoTxSpace),
            0x26 => Some(Self::NoRxSpace),
            0x27 => Some(Self::NoRxData),
            0x28 => Some(Self::NotConnected),
            0x30..=0x4A => Error::from_u8(n).map(Self::Error),
            0x50..=0x85 => Ash::from_u8(n).map(Self::Ash),
            0x86 => Some(Self::CpcErrorInit),
            0xFF => Some(Self::NoError),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        u8::try_from(n).ok().and_then(Self::from_u8)
    }
}

impl Resolve for Result<Status, u8> {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        match self {
            Ok(status) => status.ok().map_err(crate::Error::Ezsp),
            Err(status) => Err(crate::Error::InvalidEzspStatus(status)),
        }
    }
}

impl TryFrom<u8> for Status {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, <Self as TryFrom<u8>>::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
