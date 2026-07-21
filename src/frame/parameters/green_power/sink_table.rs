//! Sink table parameters.

pub use self::clear_all::Response as ClearAll;
pub use self::find_or_allocate_entry::Response as FindOrAllocateEntry;
pub use self::get_entry::Response as GetEntry;
pub use self::init::Response as Init;
pub use self::lookup::Response as Lookup;
pub use self::number_of_active_entries::Response as NumberOfActiveEntries;
pub use self::remove_entry::Response as RemoveEntry;
pub use self::set_entry::Response as SetEntry;
pub use self::set_security_frame_counter::Response as SetSecurityFrameCounter;

pub mod clear_all;
pub mod find_or_allocate_entry;
pub mod get_entry;
pub mod init;
pub mod lookup;
pub mod number_of_active_entries;
pub mod remove_entry;
pub mod set_entry;
pub mod set_security_frame_counter;

crate::frame::parameters::command_enum!(
    Command,
    ClearAll(clear_all::Command),
    FindOrAllocateEntry(find_or_allocate_entry::Command),
    GetEntry(get_entry::Command),
    Init(init::Command),
    Lookup(lookup::Command),
    NumberOfActiveEntries(number_of_active_entries::Command),
    RemoveEntry(remove_entry::Command),
    SetEntry(set_entry::Command),
    SetSecurityFrameCounter(set_security_frame_counter::Command),
);

crate::frame::parameters::parameter_enum!(
    Response,
    ClearAll,
    FindOrAllocateEntry,
    GetEntry,
    Init,
    Lookup,
    NumberOfActiveEntries,
    RemoveEntry,
    SetEntry,
    SetSecurityFrameCounter
);
