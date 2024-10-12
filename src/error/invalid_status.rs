use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InvalidStatus {
    /// An invalid [`crate::ezsp::Status`] was received.
    Ezsp(u8),
    /// An invalid [`crate::ember::Status`] was received.
    Ember(u8),
    /// An invalid [`crate::ember::duty_cycle::State`] was received.
    EmberDutyCycleState(u8),
    /// An invalid [`crate::ember::network::Status`] was received.
    EmberNetworkStatus(u8),
    /// An invalid [`crate::ember::node::Type`] was received.
    EmberNodeType(u8),
    /// An invalid [`siliconlabs::Status`] status was received.
    Siliconlabs(u32),
    /// The decision ID is invalid.
    DecisionId(u8),
}

impl Display for InvalidStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ezsp(status) => write!(f, "Invalid EZSP status: {status}"),
            Self::Ember(status) => write!(f, "Invalid Ember status: {status}"),
            Self::EmberDutyCycleState(state) => {
                write!(f, "Invalid Ember duty cycle state: {state}")
            }
            Self::EmberNetworkStatus(status) => write!(f, "Invalid Ember network status: {status}"),
            Self::EmberNodeType(node_type) => write!(f, "InvalidEmber node type: {node_type}"),
            Self::Siliconlabs(status) => write!(f, "Invalid Siliconlabs status: {status}"),
            Self::DecisionId(id) => write!(f, "Invalid decision ID: {id}"),
        }
    }
}

impl std::error::Error for InvalidStatus {}
