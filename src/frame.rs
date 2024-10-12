use std::fmt::Debug;

pub use codec::Codec;
pub use handler::Handler;
pub use header::{
    CallbackType, Command, Control, Extended, FrameFormatVersion, Header, Response, SleepMode,
    ValidControl,
};
use le_stream::derive::{FromLeStream, ToLeStream};
pub use parameters::Parameter;

mod codec;
mod handler;
mod header;
pub mod parameters;

/// A frame that contains a header and parameters.
#[derive(Debug, FromLeStream, ToLeStream)]
pub struct Frame<C, P>
where
    C: ValidControl,
    P: Parameter,
    <P as Parameter>::Id: Into<C::Size>,
{
    header: Header<C>,
    parameters: P,
}

impl<C, P> Frame<C, P>
where
    C: ValidControl,
    P: Parameter,
    <P as Parameter>::Id: Into<C::Size>,
{
    /// Create a new frame.
    #[must_use]
    pub const fn new(header: Header<C>, parameters: P) -> Self {
        Self { header, parameters }
    }

    /// Return the header.
    #[must_use]
    pub const fn header(&self) -> Header<C> {
        self.header
    }

    /// Return the parameters.
    pub fn parameters(self) -> P {
        self.parameters
    }

    /// Return the frame's codec.
    pub fn codec() -> Codec<C, P> {
        Codec::default()
    }
}
