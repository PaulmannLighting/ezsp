use core::num::TryFromIntError;

/// Invalid values.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ValueError {
    /// An invalid frame ID for a legacy header was received.
    #[error("Invalid frame ID for legacy frame format: {0}")]
    InvalidFrameId(#[from] TryFromIntError),

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
