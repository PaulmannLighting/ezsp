use std::error::Error;
use std::fmt::Display;

use crate::{ember, ezsp};

/// A status indicating an error was received from the NCP.
#[derive(Debug)]
pub enum Status {
    /// The received [`ezsp::Status`] indicates an error.
    Ezsp(Result<ezsp::Status, u8>),
    /// The received [`ember::Status`] indicates an error.
    Ember(Result<ember::Status, u8>),
    /// The received [`siliconlabs::Status`] indicates an error.
    Siliconlabs(Result<siliconlabs::Status, u32>),
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ezsp(result) => match result {
                Ok(status) => write!(f, "{status} ({status:#04X})"),
                Err(invalid) => write!(f, "Invalid EZSP status: {invalid:#04X}"),
            },
            Self::Ember(result) => match result {
                Ok(status) => write!(f, "{status} ({status:#04X})"),
                Err(invalid) => write!(f, "Invalid Ember status: {invalid:#04X}"),
            },
            Self::Siliconlabs(result) => match result {
                Ok(status) => write!(f, "{status} ({status:#010X})"),
                Err(invalid) => write!(f, "Invalid Siliconlabs status: {invalid:#010X}"),
            },
        }
    }
}

impl Error for Status {}

impl From<Result<ezsp::Status, u8>> for Status {
    fn from(result: Result<ezsp::Status, u8>) -> Self {
        Self::Ezsp(result)
    }
}

impl From<Result<ember::Status, u8>> for Status {
    fn from(result: Result<ember::Status, u8>) -> Self {
        Self::Ember(result)
    }
}

impl From<Result<siliconlabs::Status, u32>> for Status {
    fn from(result: Result<siliconlabs::Status, u32>) -> Self {
        Self::Siliconlabs(result)
    }
}

impl From<ezsp::Status> for Status {
    fn from(status: ezsp::Status) -> Self {
        Self::Ezsp(Ok(status))
    }
}

impl From<ember::Status> for Status {
    fn from(status: ember::Status) -> Self {
        Self::Ember(Ok(status))
    }
}

impl From<siliconlabs::Status> for Status {
    fn from(status: siliconlabs::Status) -> Self {
        Self::Siliconlabs(Ok(status))
    }
}
