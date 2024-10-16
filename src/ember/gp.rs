//! This module contains the types and traits for the Green Power (GP) stack.

mod address;
pub mod proxy;
pub mod security;
pub mod sink;

pub use address::Address;

/// Type alias for the GP security level.
pub type SecurityLevel = u8;

/// Type alias for the GP key type.
pub type KeyType = u8;

/// Type alias for the GP sink table entry status.
pub type SinkTableEntryStatus = u8;
