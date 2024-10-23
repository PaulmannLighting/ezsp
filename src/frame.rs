use std::fmt::Debug;

pub use handler::Handler;
pub use header::{
    CallbackType, Command, Extended, FormatVersion, Header, HighByte, Legacy, LowByte, Response,
    SleepMode,
};
use le_stream::derive::{FromLeStream, ToLeStream};
pub use parameter::Parameter;

mod handler;
mod header;
mod parameter;
pub mod parameters;
pub mod parsable;

/// A frame that contains a header and parameters.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Frame<H, P>
where
    H: Header<P::Id>,
    P: Parameter,
{
    header: H,
    parameters: P,
}

impl<H, P> Frame<H, P>
where
    H: Header<P::Id>,
    P: Parameter,
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
