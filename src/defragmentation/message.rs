//! Defragmenter outcomes.

use super::DefragmentedMessage;

/// The outcome of handling an incoming APS fragment.
#[derive(Debug)]
pub enum Message {
    /// A complete message, either directly received or reassembled.
    Complete(DefragmentedMessage),
    /// A fragment was stored or deliberately ignored.
    Incomplete,
}
