mod ash;
mod error;
pub mod network;
mod spi_err;

use anyhow::anyhow;
pub use ash::Ash;
pub use error::Error;
use num_traits::{FromPrimitive, ToPrimitive};
pub use spi_err::SpiErr;

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

impl ToPrimitive for Status {
    fn to_i64(&self) -> Option<i64> {
        self.to_u8().map(i64::from)
    }

    fn to_u8(&self) -> Option<u8> {
        match self {
            Self::Success => Some(0x00),
            Self::SpiErr(spi_err) => spi_err.to_u8(),
            Self::AshInProgress => Some(0x20),
            Self::HostFatalError => Some(0x21),
            Self::AshNcpFatalError => Some(0x22),
            Self::DataFrameTooLong => Some(0x23),
            Self::DataFrameTooShort => Some(0x24),
            Self::NoTxSpace => Some(0x25),
            Self::NoRxSpace => Some(0x26),
            Self::NoRxData => Some(0x27),
            Self::NotConnected => Some(0x28),
            Self::Error(error) => error.to_u8(),
            Self::Ash(ash) => ash.to_u8(),
            Self::CpcErrorInit => Some(0x86),
            Self::NoError => Some(0xFF),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_u8().map(u64::from)
    }
}

impl From<Status> for u8 {
    fn from(status: Status) -> Self {
        status.to_u8().expect("could not convert Status to u8")
    }
}

impl TryFrom<u8> for Status {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, <Self as TryFrom<u8>>::Error> {
        Self::from_u8(value).ok_or_else(|| anyhow!("Invalid EzspStatus: {value:#04X}"))
    }
}
