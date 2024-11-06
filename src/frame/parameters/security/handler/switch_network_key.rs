use le_stream::derive::FromLeStream;

use crate::frame::Parameter;

const ID: u16 = 0x006E;

/// A callback to inform the application that the Network Key has been updated
/// and the node has been switched over to use the new key.
///
/// The actual key being used is not passed up, but the sequence number is.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    sequence_number: u8,
}

impl Handler {
    /// The sequence number of the new network key.
    #[must_use]
    pub const fn sequence_number(&self) -> u8 {
        self.sequence_number
    }
}

impl Parameter for Handler {
    const ID: u16 = ID;
}
