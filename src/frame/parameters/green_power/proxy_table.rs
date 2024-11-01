//! Proxy table parameters.

pub mod get_entry;
pub mod lookup;
pub mod process_gp_pairing;

/// Green Power sink table response parameters.
#[allow(variant_size_differences)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Response to the `sink_table::get_entry` command.
    GetEntry(Box<get_entry::Response>),
    /// Response to the `sink_table::lookup` command.
    Lookup(lookup::Response),
    /// Response to the `sink_table::process_gp_pairing` command.
    ProcessGpPairing(process_gp_pairing::Response),
}
