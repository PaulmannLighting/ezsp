use core::fmt::Debug;

pub use self::enums::{Callback, Command as Commands, Parameters, Response};
pub use self::header::{
    CallbackType, Command, Extended, FormatVersion, Header, HighByte, Legacy, LowByte, SleepMode,
};
pub use self::parameter::Parameter;
pub use self::parsable::Parsable;
pub use self::responds_with::RespondsWith;

mod enums;
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
pub struct Frame<T> {
    header: Header,
    payload: T,
}

impl<T> Frame<T> {
    /// Create a new frame.
    #[must_use]
    pub const fn new(header: Header, payload: T) -> Self {
        Self { header, payload }
    }
}

impl<T> From<(Header, T)> for Frame<T> {
    fn from((header, payload): (Header, T)) -> Self {
        Self { header, payload }
    }
}

impl<T> From<Frame<T>> for (Header, T) {
    fn from(frame: Frame<T>) -> Self {
        (frame.header, frame.payload)
    }
}
