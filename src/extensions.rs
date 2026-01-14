//! Extension traits for various types.
//!
//!
use std::fmt::Display;

pub use self::configuration_ext::ConfigurationExt;
pub use self::policy_ext::PolicyExt;
mod configuration_ext;
mod policy_ext;

/// Extension trait for converting a type into a displayable form.
pub trait Displayable {
    /// The displayable form of the type.
    type Displayable: Display;

    /// Converts the type into its displayable form.
    fn displayable(self) -> Self::Displayable;
}
