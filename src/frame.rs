mod header;
pub mod parameters;

pub use header::{CallbackType, Control, FrameFormatVersion, Header, HighByte, LowByte, SleepMode};
use le_stream::derive::{FromLeBytes, ToLeBytes};
use le_stream::{FromLeBytes, ToLeBytes};
pub use parameters::Parameter;
use std::fmt::Debug;

#[derive(Debug, FromLeBytes, ToLeBytes)]
pub struct Frame<P>
where
    P: Parameter,
{
    header: Header<P::Id>,
    parameters: P,
}

impl<P> Frame<P>
where
    P: Parameter,
{
    #[must_use]
    pub const fn new(header: Header<P::Id>, parameters: P) -> Self {
        Self { header, parameters }
    }
    pub fn parameters(self) -> P {
        self.parameters
    }
}
