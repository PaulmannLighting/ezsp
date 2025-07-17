use crate::Error;

/// A specialized [`Result`] type for this crate.
pub type Result<T> = core::result::Result<T, Error>;
