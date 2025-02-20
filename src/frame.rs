use std::fmt::Debug;

pub use disambiguation::Disambiguation;
pub use enums::{Callback, Parameters, Response};
pub use header::{
    CallbackType, Command, Extended, FormatVersion, Header, HighByte, Legacy, LowByte, SleepMode,
};
pub use parameter::Parameter;
pub use parsable::Parsable;

mod disambiguation;
mod enums;
mod header;
mod parameter;
pub mod parameters;
pub mod parsable;

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
}

impl From<(Header, Parameters)> for Frame {
    fn from((header, parameters): (Header, Parameters)) -> Self {
        Self { header, parameters }
    }
}

impl From<Frame> for (Header, Parameters) {
    fn from(frame: Frame) -> Self {
        (frame.header, frame.parameters)
    }
}
