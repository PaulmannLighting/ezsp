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
    type Id;
    const ID: Self::Id;
    const DISAMBIGUATION: Option<Disambiguation> = None;
    const UNIQUE_ID: (Self::Id, Option<Disambiguation>) = (Self::ID, Self::DISAMBIGUATION);
}
