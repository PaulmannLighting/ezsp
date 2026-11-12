//! Extension traits for various types.
//!
//!
use std::fmt::Display;

pub use self::configuration_ext::ConfigurationExt;
pub use self::identity::{Identity, IdentityChange, IdentityError, IdentityStage};
pub use self::policy_ext::PolicyExt;
pub use self::security_counters::SecurityCounters;
pub use self::security_ext::{NetworkKeyExport, SecurityExt};
mod configuration_ext;
mod identity;
mod policy_ext;
mod security_counters;
mod security_ext;

/// Extension trait for converting a type into a displayable form.
pub trait Displayable {
    /// The displayable form of the type.
    type Displayable: Display;

    /// Converts the type into its displayable form.
    fn displayable(self) -> Self::Displayable;
}
