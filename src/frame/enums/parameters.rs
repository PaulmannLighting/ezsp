use super::{Callback, Response};

mod conversion;
/// Conversion traits for `Parameters`.
mod parsing;

/// Parameter types of `EZSP` frames.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Parameters {
    /// A response.
    Response(Response),
    /// A callback.
    Callback(Callback),
}
