pub mod header;
pub mod parameters;

pub use header::Header;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use le_stream::{FromLeBytes, ToLeBytes};
use std::fmt::Debug;

#[derive(Debug, FromLeBytes, ToLeBytes)]
pub struct Frame<C, I, P>
where
    C: Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    I: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    P: FromLeBytes + ToLeBytes,
{
    header: Header<C, I>,
    parameters: P,
}

impl<C, I, P> Frame<C, I, P>
where
    C: Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    I: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    P: FromLeBytes + ToLeBytes,
{
    pub fn parameters(self) -> P {
        self.parameters
    }
}
