use core::error::Error;
use core::fmt;
use core::fmt::Display;

/// Error that can occur during the defragmentation of EZSP frames.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DefragmentationError {
    /// Indicates that a non-initial EZSP fragment was received,
    /// which cannot be associated with a defragmentation.
    StrayFragment {
        /// APS sequence number.
        seq: u8,
        /// Fragmentation index.
        index: u8,
    },
    /// Indicates that a duplicate fragment has been received.
    ///
    /// This may indicate a race condition between frame
    /// defragmentation and sequence counter wrapping.
    DuplicateFragment {
        /// APS sequence number.
        seq: u8,
        /// Fragmentation index.
        index: u8,
    },
}

impl Display for DefragmentationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateFragment { seq, index } => {
                write!(
                    f,
                    "Duplicate defragmentation sequence at index {index} with sequence {seq}."
                )
            }
            Self::StrayFragment { seq, index } => {
                write!(f, "Stray fragment at index {index} with sequence {seq}.")
            }
        }
    }
}

impl Error for DefragmentationError {}
