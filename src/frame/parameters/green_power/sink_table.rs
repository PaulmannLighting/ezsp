//! Sink table parameters.

pub mod clear_all;
pub mod find_or_allocate_entry;
pub mod get_entry;
pub mod init;
pub mod lookup;
pub mod number_of_active_entries;
pub mod remove_entry;
pub mod set_entry;
pub mod set_security_frame_counter;

/// Sink table response parameters.
#[expect(variant_size_differences)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Response to the `sink_table_clear_all` command.
    ClearAll(clear_all::Response),
    /// Response to the `sink_table_find_or_allocate_entry` command.
    FindOrAllocateEntry(find_or_allocate_entry::Response),
    /// Response to the `sink_table_get_entry` command.
    GetEntry(Box<get_entry::Response>),
    /// Response to the `sink_table_init` command.
    Init(init::Response),
    /// Response to the `sink_table_lookup` command.
    Lookup(lookup::Response),
    /// Response to the `sink_table_number_of_active_entries` command.
    NumberOfActiveEntries(number_of_active_entries::Response),
    /// Response to the `sink_table_remove_entry` command.
    RemoveEntry(remove_entry::Response),
    /// Response to the `sink_table_set_entry` command.
    SetEntry(set_entry::Response),
    /// Response to the `sink_table_set_security_frame_counter` command.
    SetSecurityFrameCounter(set_security_frame_counter::Response),
}
