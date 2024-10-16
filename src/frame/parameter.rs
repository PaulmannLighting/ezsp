use std::fmt::{Debug, Display, LowerHex, UpperHex};
use std::hash::Hash;

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
}
