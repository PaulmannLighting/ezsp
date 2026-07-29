//! Error conversion for the `apis-saltans` driver boundary.
//!
//! EZSP errors retain their concrete value inside the hardware abstraction's
//! backend error variant without discarding their source or display text.

impl From<crate::Error> for apis_saltans_hw::Error {
    fn from(error: crate::Error) -> Self {
        Self::backend(error)
    }
}
