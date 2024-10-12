/// Callback types.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CallbackType {
    /// Reserved.
    Reserved,
    /// Async callback actively sent by the NCP.
    Async,
    /// Sync callback actively requested by the host.
    Sync,
}
