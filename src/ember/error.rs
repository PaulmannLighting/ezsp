use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidCounterType(u8),
    InvalidEntropySource(u8),
    InvalidEventUnit(u8),
    InvalidJoinMethod(u8),
    InvalidNetworkInitBitmask(u16),
    InvalidNetworkStatus(u8),
    InvalidNodeType(u8),
    InvalidStatus(u8),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCounterType(typ) => write!(f, "invalid counter type: {typ}"),
            Self::InvalidEntropySource(entropy_source) => {
                write!(f, "invalid entropy source: {entropy_source:#04X}")
            }
            Self::InvalidEventUnit(unit) => write!(f, "invalid event unit: {unit:#04X}"),
            Self::InvalidJoinMethod(join_method) => {
                write!(f, "invalid join method: {join_method:#04X}")
            }
            Self::InvalidNetworkInitBitmask(bitmask) => {
                write!(f, "invalid network init bitmask: {bitmask:#04X}")
            }
            Self::InvalidNetworkStatus(status) => {
                write!(f, "invalid network status: {status:#04X}")
            }
            Self::InvalidNodeType(typ) => write!(f, "invalid node type: {typ:#04X}"),
            Self::InvalidStatus(status) => write!(f, "invalid status: {status:#04X}"),
        }
    }
}

impl std::error::Error for Error {}
