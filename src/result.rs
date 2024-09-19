use crate::Error;

/// A specialized [`Result`] type for this crate.
pub type Result<T> = std::result::Result<T, Error>;
