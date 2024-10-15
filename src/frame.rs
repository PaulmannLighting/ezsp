use std::fmt::Debug;

pub use handler::Handler;
pub use header::{
    CallbackType, Command, Extended, FrameFormatVersion, Header, Legacy, Response, SleepMode,
};
use le_stream::derive::{FromLeStream, ToLeStream};
pub use parameters::Parameter;

mod handler;
mod header;
pub mod parameters;

/// A frame that contains a header and parameters.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Frame<H, P>
where
    H: Header<P::Id>,
    P: Parameter,
    u16: From<P::Id>,
{
    header: H,
    parameters: P,
}

impl<H, P> Frame<H, P>
where
    H: Header<P::Id>,
    P: Parameter,
    u16: From<P::Id>,
{
    /// Create a new frame.
    #[must_use]
    pub const fn new(header: H, parameters: P) -> Self {
        Self { header, parameters }
    }

    /// Return the header.
    #[must_use]
    pub const fn header(&self) -> H {
        self.header
    }

    /// Return the parameters.
    pub fn parameters(self) -> P {
        self.parameters
    }
}
