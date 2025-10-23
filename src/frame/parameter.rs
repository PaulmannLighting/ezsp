use core::fmt::Debug;

use super::disambiguation::Disambiguation;

pub trait Parameter: Debug + Send {
    /// The frame ID.
    const ID: u16;

    /// An optional disambiguation.
    ///
    /// This is necessary since there are multiple frames with the same ID.
    const DISAMBIGUATION: Disambiguation = Disambiguation::None;

    /// The unique ID of the frame consisting of the frame ID and the optional disambiguation.
    const UNIQUE_ID: (u16, Disambiguation) = (Self::ID, Self::DISAMBIGUATION);
}
