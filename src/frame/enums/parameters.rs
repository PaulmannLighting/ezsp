use super::{Callback, Response};

mod conversion;
mod parsing;

/// This enum represents the parameters of `EZSP` frames.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Parameters {
    Response(Response),
    Handler(Callback),
}
