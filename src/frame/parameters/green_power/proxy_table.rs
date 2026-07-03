//! Proxy table parameters.

pub use self::get_entry::Response as GetEntry;
pub use self::lookup::Response as Lookup;
pub use self::process_gp_pairing::Response as ProcessGpPairing;

pub mod get_entry;
pub mod lookup;
pub mod process_gp_pairing;

crate::frame::parameters::parameter_enum!(Response, GetEntry, Lookup, ProcessGpPairing);
