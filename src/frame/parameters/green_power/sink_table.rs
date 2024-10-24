pub mod clear_all;
pub mod find_or_allocate_entry;
pub mod get_entry;
pub mod init;
pub mod lookup;
pub mod number_of_active_entries;
pub mod remove_entry;
pub mod set_entry;
pub mod set_security_frame_counter;

#[allow(variant_size_differences)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    ClearAll(clear_all::Response),
    FindOrAllocateEntry(find_or_allocate_entry::Response),
    GetEntry(Box<get_entry::Response>),
    Init(init::Response),
    Lookup(lookup::Response),
    NumberOfActiveEntries(number_of_active_entries::Response),
    RemoveEntry(remove_entry::Response),
    SetEntry(set_entry::Response),
    SetSecurityFrameCounter(set_security_frame_counter::Response),
}
