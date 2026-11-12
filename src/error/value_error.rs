use core::num::TryFromIntError;
use std::io;
use std::io::ErrorKind;

/// Invalid values.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ValueError {
    /// A token payload exceeds the supported storage capacity.
    #[error("Invalid token payload size: {0}")]
    InvalidTokenSize(u32),

    /// A security counter response has an unexpected byte length.
    #[error("Invalid security counter length: {0}")]
    InvalidSecurityCounterLength(usize),

    /// A security counter write was requested while the NCP was not offline.
    #[error("Cannot set security counters in network state {0:#04X}")]
    SecurityCounterWriteWhileOnline(u8),

    /// The active network key is not set.
    #[error("No active network key is set")]
    MissingNetworkKey,

    /// Network-key metadata changed during export.
    #[error("Network-key metadata changed during export")]
    NetworkKeyChanged,

    /// An invalid frame ID for a legacy header was received.
    #[error("Invalid frame ID for legacy frame format: {0}")]
    InvalidFrameId(#[from] TryFromIntError),

    /// A configured route radius does not fit into an EZSP route request.
    #[error("Invalid route radius: {0}")]
    InvalidRouteRadius(u16),

    /// An invalid [`State`](crate::ember::duty_cycle::State) was received.
    #[error("Invalid Ember duty cycle state: {0:#04X}")]
    EmberDutyCycleState(u8),

    /// An invalid [`Status`](crate::ember::network::Status) was received.
    #[error("Invalid Ember network status: {0:#04X}")]
    EmberNetworkStatus(u8),

    /// An invalid [`Type`](crate::ember::node::Type) was received.
    #[error("InvalidEmber node type: {0:#04X}")]
    EmberNodeType(u8),

    /// The decision ID is invalid.
    #[error("Invalid decision ID: {0:#04X}")]
    DecisionId(u8),

    /// The decision ID is invalid.
    #[error("Invalid entropy source: {0:#04X}")]
    EntropySource(u8),

    /// Indicates that some expected payload was missing.
    #[error("Missing payload")]
    MissingPayload,
}

impl From<ValueError> for io::Error {
    fn from(error: ValueError) -> Self {
        let kind = match error {
            ValueError::InvalidRouteRadius(_)
            | ValueError::InvalidTokenSize(_)
            | ValueError::SecurityCounterWriteWhileOnline(_) => ErrorKind::InvalidInput,
            ValueError::MissingNetworkKey
            | ValueError::NetworkKeyChanged
            | ValueError::InvalidSecurityCounterLength(_)
            | ValueError::InvalidFrameId(_)
            | ValueError::EmberDutyCycleState(_)
            | ValueError::EmberNetworkStatus(_)
            | ValueError::EmberNodeType(_)
            | ValueError::DecisionId(_)
            | ValueError::EntropySource(_)
            | ValueError::MissingPayload => ErrorKind::InvalidData,
        };

        Self::new(kind, error)
    }
}
