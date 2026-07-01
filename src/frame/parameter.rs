use core::fmt::Debug;

/// Trait to identify parameters with a unique ID.
pub trait Parameter: Debug + Send {
    /// The frame ID.
    const ID: u16;
}
