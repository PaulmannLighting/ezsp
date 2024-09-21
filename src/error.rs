pub mod value;

use std::fmt::{Debug, Display, Formatter};

use crate::frame::parameters::utilities::invalid_command;
use crate::{ember, ezsp};

/// An error that can occur when communicating with an NCP.
#[derive(Debug)]
pub enum Error {
    /// An `ASHv2` protocol-related error occurred.
    #[cfg(feature = "ashv2")]
    Ashv2(ashv2::Error),
    /// An invalid [`ezsp::Status`] was received.
    InvalidEzspStatus(u8),
    /// An invalid [`ember::Status`] was received.
    InvalidEmberStatus(u8),
    /// An invalid [`ember::duty_cycle::State`] was received.
    InvalidEmberDutyCycleState(u8),
    /// An invalid [`ember::network::Status`] was received.
    InvalidEmberNetworkStatus(u8),
    /// An invalid [`ember::node::Type`] was received.
    InvalidEmberNodeType(u8),
    /// An invalid [`siliconlabs::Status`] status was received.
    InvalidSiliconlabsStatus(u32),
    /// The received [`ezsp::Status`] indicates an error.
    Ezsp(ezsp::Status),
    /// The received [`ember::Status`] indicates an error.
    Ember(ember::Status),
    /// The received [`siliconlabs::Status`] indicates an error.
    Siliconlabs(siliconlabs::Status),
    /// The read data was incomplete.
    IncompleteData(le_stream::Error),
    /// A value-related error occurred.
    ValueError(value::Error),
    /// The header is invalid.
    InvalidHeader {
        /// The expected header.
        expected: u16,
        /// The found header.
        found: u16,
    },
    /// The command is invalid.
    InvalidCommand(invalid_command::Response),
    /// A custom error message.
    Custom(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "ashv2")]
            Self::Ashv2(error) => Display::fmt(error, f),
            Self::InvalidEzspStatus(status) => write!(f, "Invalid EZSP status: {status}"),
            Self::InvalidEmberStatus(status) => write!(f, "Invalid Ember status: {status}"),
            Self::InvalidEmberDutyCycleState(state) => {
                write!(f, "Invalid Ember duty cycle state: {state}")
            }
            Self::InvalidEmberNetworkStatus(status) => {
                write!(f, "Invalid Ember network status: {status}")
            }
            Self::InvalidEmberNodeType(node_type) => {
                write!(f, "Invalid Ember node type: {node_type}")
            }
            Self::InvalidSiliconlabsStatus(status) => {
                write!(f, "Invalid Siliconlabs status: {status}")
            }
            Self::Ezsp(status) => write!(f, "Ezsp: {}", u8::from(*status)),
            Self::Ember(status) => write!(f, "Ember: {}", u8::from(*status)),
            Self::Siliconlabs(status) => write!(f, "Siliconlabs: {}", u32::from(*status)),
            Self::IncompleteData(error) => Display::fmt(error, f),
            Self::ValueError(error) => Display::fmt(error, f),
            Self::InvalidHeader { expected, found } => {
                write!(f, "Invalid header: expected {expected}, found {found}")
            }
            Self::InvalidCommand(response) => write!(f, "Invalid command: {:?}", response.reason()),
            Self::Custom(msg) => Display::fmt(msg, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            #[cfg(feature = "ashv2")]
            Self::Ashv2(error) => Some(error),
            Self::IncompleteData(error) => Some(error),
            Self::ValueError(error) => Some(error),
            _ => None,
        }
    }
}

#[cfg(feature = "ashv2")]
impl From<ashv2::Error> for Error {
    fn from(error: ashv2::Error) -> Self {
        Self::Ashv2(error)
    }
}

impl From<ezsp::Status> for Error {
    fn from(status: ezsp::Status) -> Self {
        Self::Ezsp(status)
    }
}

impl From<ember::Status> for Error {
    fn from(status: ember::Status) -> Self {
        Self::Ember(status)
    }
}

impl From<value::Error> for Error {
    fn from(error: value::Error) -> Self {
        Self::ValueError(error)
    }
}

impl From<le_stream::Error> for Error {
    fn from(error: le_stream::Error) -> Self {
        Self::IncompleteData(error)
    }
}

impl From<invalid_command::Response> for Error {
    fn from(response: invalid_command::Response) -> Self {
        Self::InvalidCommand(response)
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self::Custom(msg)
    }
}
