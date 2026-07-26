//! Errors produced while translating an incoming EZSP APS message.

/// An error that can occur when parsing an APS frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash, thiserror::Error)]
pub enum ParseApsFrameError {
    /// Invalid message type.
    #[error("Invalid message type: {0}")]
    MessageType(u8),
}
