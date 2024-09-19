use std::fmt::Debug;

use le_stream::derive::{FromLeStream, ToLeStream};

pub use handler::Handler;
pub use header::{CallbackType, Control, FrameFormatVersion, Header, HighByte, LowByte, SleepMode};
pub use parameters::Parameter;

mod handler;
mod header;
pub mod parameters;

/// A frame that contains a header and parameters.
#[derive(Debug, FromLeStream, ToLeStream)]
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
    /// Create a new frame.
    #[must_use]
    pub const fn new(header: Header<P::Id>, parameters: P) -> Self {
        Self { header, parameters }
    }

    /// Return the header.
    #[must_use]
    pub const fn header(&self) -> &Header<P::Id> {
        &self.header
    }

    /// Return the parameters.
    pub fn parameters(self) -> P {
        self.parameters
    }
}
