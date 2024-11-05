use std::fmt::{Debug, Display, LowerHex, UpperHex};
use std::hash::Hash;

use crate::frame::disambiguation::Disambiguation;
use le_stream::{FromLeStream, ToLeStream};

pub trait Identified: Debug + Send
where
    <Self as Identified>::Id: Copy
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
}

impl<T> Parameter for T
where
    T: Identified,
{
    type Id = <Self as Identified>::Id;
}
