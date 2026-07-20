use apis_saltans_hw::core::endpoint::Reserved;

/// An error that can occur when parsing an APS frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash, thiserror::Error)]
pub enum ParseApsFrameError {
    /// Invalid message type.
    #[error("Invalid message type: {0}")]
    MessageType(u8),

    /// The APS destination endpoint is invalid.
    #[error("Invalid APS application endpoint ID: {0:#04X}")]
    InvalidEndpoint(Reserved),

    /// The APS group ID is invalid.
    #[error("Invalid APS group ID: {0}")]
    GroupId(u16),

    /// The APS source endpoint ID is invalid.
    #[error("Invalid APS source endpoint ID: {0}")]
    SourceEndpoint(u8),

    /// The profile ID is invalid.
    #[error("Invalid profile ID: {0}")]
    Profile(u16),
}
