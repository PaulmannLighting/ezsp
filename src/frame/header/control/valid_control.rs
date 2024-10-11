use super::extended::Extended;
use super::{Command, Response};
use crate::frame::header::control::low_byte::LowByte;
use le_stream::{FromLeStream, ToLeStream};
use std::fmt::{Debug, Display, LowerHex, UpperHex};
use std::hash::Hash;

pub trait ValidControl:
    Copy + Clone + Debug + Default + Eq + Hash + FromLeStream + ToLeStream
{
    type Size: Copy
        + Debug
        + Display
        + Eq
        + Into<u16>
        + LowerHex
        + UpperHex
        + FromLeStream
        + ToLeStream;
}

impl ValidControl for LowByte {
    type Size = u8;
}

impl ValidControl for Extended<LowByte> {
    type Size = u16;
}
