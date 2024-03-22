pub mod header;
pub mod parameters;

pub use header::Header;
use le_stream::derive::FromLeBytes;
use le_stream::{FromLeBytes, ToLeBytes};
use std::fmt::Debug;

#[derive(Debug, FromLeBytes)]
pub struct ResponseFrame<C, I, P>
where
    C: Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    I: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    P: FromLeBytes,
{
    header: Header<C, I>,
    parameters: P,
}

impl<C, I, P> ResponseFrame<C, I, P>
where
    C: Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    I: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    P: FromLeBytes,
{
    pub fn parameters(self) -> P {
        self.parameters
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    //todo!()
}

#[derive(Debug, Eq, PartialEq)]
pub enum Response {
    //todo!()
}

#[derive(Debug, Eq, PartialEq)]
pub enum Callback {
    //todo!()
}
