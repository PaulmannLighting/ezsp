pub mod get_entry;
pub mod lookup;
pub mod process_gp_pairing;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    GetEntry(get_entry::Response),
    Lookup(lookup::Response),
    ProcessGpPairing(process_gp_pairing::Response),
}
