mod address;
pub mod proxy;
pub mod security;
pub mod sink;

pub use address::Address;

pub type SecurityLevel = u8;
pub type KeyType = u8;
pub type SinkTableEntryStatus = u8;
