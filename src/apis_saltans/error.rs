//! Error conversion for the `apis-saltans` driver boundary.
//!
//! EZSP errors retain their concrete value inside the hardware abstraction's
//! implementation-specific error variant. An [`Arc`] makes the resulting error
//! cloneable without discarding its source or display text.

use std::sync::Arc;

impl From<crate::Error> for apis_saltans_hw::Error {
    fn from(error: crate::Error) -> Self {
        Self::Implementation(Arc::new(error))
    }
}
