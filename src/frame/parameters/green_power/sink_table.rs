pub mod clear_all;
pub mod find_or_allocate_entry;
pub mod get_entry;
pub mod init;
pub mod lookup;
pub mod number_of_active_entries;
pub mod remove_entry;
pub mod set_entry;
pub mod set_security_frame_counter;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    ClearAll(clear_all::Command),
    FindOrAllocateEntry(find_or_allocate_entry::Command),
    GetEntry(get_entry::Command),
    Init(init::Command),
    Lookup(lookup::Command),
    NumberOfActiveEntries(number_of_active_entries::Command),
    RemoveEntry(remove_entry::Command),
    SetEntry(set_entry::Command),
    SetSecurityFrameCounter(set_security_frame_counter::Command),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    ClearAll(clear_all::Response),
    FindOrAllocateEntry(find_or_allocate_entry::Response),
    GetEntry(get_entry::Response),
    Init(init::Response),
    Lookup(lookup::Response),
    NumberOfActiveEntries(number_of_active_entries::Response),
    RemoveEntry(remove_entry::Response),
    SetEntry(set_entry::Response),
    SetSecurityFrameCounter(set_security_frame_counter::Response),
}
