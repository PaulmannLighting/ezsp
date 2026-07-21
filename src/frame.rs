use core::fmt::Debug;

pub use self::enums::{Callback, Parameters, Response};
pub use self::header::{
    CallbackType, Command, Extended, FormatVersion, Header, HighByte, Legacy, LowByte, SleepMode,
};
pub use self::parameter::Parameter;
pub use self::parsable::Parsable;
pub use self::responds_with::RespondsWith;

pub mod enums;
mod header;
mod parameter;
pub mod parameters;
pub mod parsable;
mod responds_with;

/// A decoded EZSP frame.
///
/// An EZSP frame consists of a sequence number, frame-control bytes, a frame
/// ID, and frame-specific parameters. The header stores the sequence/control/ID
/// fields, while [`Parameters`] stores either response parameters or callback
/// parameters.
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
