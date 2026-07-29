//! Errors produced while translating an incoming EZSP APS message.

/// An error that can occur when parsing an APS frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash, thiserror::Error)]
pub enum ParseApsFrameError {
    /// Invalid message type.
    #[error("Invalid message type: {0}")]
    MessageType(u8),

    /// The sender used a reserved or broadcast NWK address.
    #[error("Invalid APSDE source network address: {0:#06X}")]
    SourceAddress(u16),

    /// The APS source endpoint was the broadcast endpoint.
    #[error("Invalid individual APS source endpoint: {0:#04X}")]
    SourceEndpoint(u8),

    /// The APS destination endpoint was the broadcast endpoint.
    #[error("Invalid individual APS destination endpoint: {0:#04X}")]
    DestinationEndpoint(u8),

    /// The multicast callback carried an invalid group identifier.
    #[error("Invalid APS group identifier: {0:#06X}")]
    GroupId(u16),
}
