use std::fmt::{Debug, Display, LowerHex, UpperHex};
use std::hash::Hash;

use crate::frame::disambiguation::Disambiguation;
use le_stream::{FromLeStream, ToLeStream};

pub trait Parameter: Debug + Send
where
    <Self as Parameter>::Id: Copy
        + Debug
        + Display
        + Eq
        + Hash
        + Into<u16>
        + LowerHex
        + UpperHex
        + Send
        + FromLeStream
        + ToLeStream,
{
    /// The type of the frame ID.
    ///
    /// This is usually `u16`, but may as well be `u8`.
    type Id;

    /// The frame ID.
    const ID: Self::Id;

    /// An optional disambiguation.
    ///
    /// This is necessary since there are multiple frames with the same ID.
    const DISAMBIGUATION: Option<Disambiguation> = None;

    /// The unique ID of the frame consisting of the frame ID and the optional disambiguation.
    const UNIQUE_ID: (Self::Id, Option<Disambiguation>) = (Self::ID, Self::DISAMBIGUATION);
}
