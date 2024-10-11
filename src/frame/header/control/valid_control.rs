use super::extended::Extended;
use super::{Command, Response};
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

impl ValidControl for Command {
    type Size = u8;
}

impl ValidControl for Response {
    type Size = u8;
}

impl ValidControl for Extended<Command> {
    type Size = u16;
}

impl ValidControl for Extended<Response> {
    type Size = u16;
}
