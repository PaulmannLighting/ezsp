use crate::Parameters;

/// Trait to identify frames that respond with a `Response`.
pub trait RespondsWith {
    /// The response type.
    type Response: TryFrom<Parameters, Error: Into<Parameters> + Send> + Send;
}
