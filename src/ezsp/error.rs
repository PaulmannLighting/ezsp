use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidConfigId(u8),
    InvalidCounter(u8),
    InvalidDecisionBitmask(u8),
    InvalidDecisionId(u8),
    InvalidExtendedId(u8),
    InvalidNetworkInitBitmask(u16),
    InvalidNetworkScanType(u8),
    InvalidNetworkStatus(u8),
    InvalidPolicyId(u8),
    InvalidStatus(u8),
    InvalidTokenId(u8),
    InvalidValueId(u8),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidConfigId(id) => write!(f, "invalid config id: {id:#04X}"),
            Self::InvalidCounter(counter) => write!(f, "invalid counter: {counter:#04X}"),
            Self::InvalidDecisionBitmask(bitmask) => {
                write!(f, "invalid decision bitmask: {bitmask:#04X}")
            }
            Self::InvalidDecisionId(id) => write!(f, "invalid decision id: {id:#04X}"),
            Self::InvalidExtendedId(id) => write!(f, "invalid extended id: {id:#04X}"),
            Self::InvalidNetworkInitBitmask(bitmask) => {
                write!(f, "invalid network init bitmask: {bitmask:#04X}")
            }
            Self::InvalidNetworkScanType(typ) => write!(f, "invalid network scan type: {typ:#04X}"),
            Self::InvalidNetworkStatus(status) => {
                write!(f, "invalid network status: {status:#04X}")
            }
            Self::InvalidPolicyId(id) => write!(f, "invalid policy id: {id:#04X}"),
            Self::InvalidStatus(status) => write!(f, "invalid status: {status:#04X}"),
            Self::InvalidTokenId(id) => write!(f, "invalid token id: {id:#04X}"),
            Self::InvalidValueId(id) => write!(f, "invalid value id: {id:#04X}"),
        }
    }
}

impl std::error::Error for Error {}
