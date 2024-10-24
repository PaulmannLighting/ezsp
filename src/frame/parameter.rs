use std::fmt::{Debug, Display, LowerHex, UpperHex};
use std::hash::Hash;

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
