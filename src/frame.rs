mod header;
pub mod parameters;

pub use header::{CallbackType, Control, FrameFormatVersion, Header, HighByte, LowByte, SleepMode};
use le_stream::derive::{FromLeBytes, ToLeBytes};
use le_stream::{FromLeBytes, ToLeBytes};
pub use parameters::Parameter;
use std::fmt::Debug;

#[derive(Debug, FromLeBytes, ToLeBytes)]
pub struct Frame<C, I, P>
where
    C: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    I: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    P: FromLeBytes + ToLeBytes,
{
    header: Header<C, I>,
    parameters: P,
}

impl<C, I, P> Frame<C, I, P>
where
    C: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    I: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
    P: FromLeBytes + ToLeBytes,
{
    #[must_use]
    pub const fn new(header: Header<C, I>, parameters: P) -> Self {
        Self { header, parameters }
    }
    pub fn parameters(self) -> P {
        self.parameters
    }
}
