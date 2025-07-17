use core::fmt::Display;
use core::num::TryFromIntError;

/// Invalid values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueError {
    /// An invalid frame ID for a legacy header was received.
    InvalidFrameId(TryFromIntError),
    /// An invalid [`crate::ember::duty_cycle::State`] was received.
    EmberDutyCycleState(u8),
    /// An invalid [`crate::ember::network::Status`] was received.
    EmberNetworkStatus(u8),
    /// An invalid [`crate::ember::node::Type`] was received.
    EmberNodeType(u8),
    /// The decision ID is invalid.
    DecisionId(u8),
    /// The decision ID is invalid.
    EntropySource(u8),
    /// Indicates that some expected payload was missing.
    MissingPayload,
}

impl Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFrameId(error) => {
                write!(f, "Invalid frame ID for legacy frame format: {error}")
            }
            Self::EmberDutyCycleState(state) => {
                write!(f, "Invalid Ember duty cycle state: {state:#04X}")
            }
            Self::EmberNetworkStatus(status) => {
                write!(f, "Invalid Ember network status: {status:#04X}")
            }
            Self::EmberNodeType(node_type) => write!(f, "InvalidEmber node type: {node_type:#04X}"),
            Self::DecisionId(id) => write!(f, "Invalid decision ID: {id:#04X}"),
            Self::EntropySource(source) => write!(f, "Invalid entropy source: {source:#04X}"),
            Self::MissingPayload => write!(f, "Missing payload"),
        }
    }
}

impl std::error::Error for ValueError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidFrameId(error) => Some(error),
            _ => None,
        }
    }
}

impl From<TryFromIntError> for ValueError {
    fn from(error: TryFromIntError) -> Self {
        Self::InvalidFrameId(error)
    }
}
