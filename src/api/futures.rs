/// Futures that drive the EZSP transmitter and receiver actors.
///
/// [`Client::run`](crate::Client::run) returns these futures with the newly
/// wired client. Spawn both futures before connecting or passing that client to
/// [`Builder`](crate::Builder). The transport implementation must first start
/// any lower-level I/O tasks on which these actors depend.
///
/// This type is available as `ezsp::Futures` from the crate root.
pub struct Futures<T, R> {
    /// Future that serializes outbound commands and correlates inbound responses.
    pub transmitter: T,
    /// Future that routes decoded frames to response or callback channels.
    pub receiver: R,
}
