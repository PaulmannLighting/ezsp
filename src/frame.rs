use std::fmt::Debug;

pub use handler::Callback;
pub use header::{
    CallbackType, Command, Extended, FormatVersion, Header, HighByte, Legacy, LowByte, SleepMode,
};
pub use parameter::{Identified, Parameter};
pub use parameters::Parameters;
pub use parsable::Parsable;

mod handler;
mod header;
mod parameter;
pub mod parameters;
pub mod parsable;
pub mod response;

/// A frame that contains a header and parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Frame {
    header: Header,
    parameters: Parameters,
}

impl Frame {
    /// Create a new frame.
    #[must_use]
    pub const fn new(header: Header, parameters: Parameters) -> Self {
        Self { header, parameters }
    }

    /// Return the header.
    #[must_use]
    pub const fn header(&self) -> Header {
        self.header
    }

    /// Return the parameters.
    pub fn parameters(self) -> Parameters {
        self.parameters
    }
}
