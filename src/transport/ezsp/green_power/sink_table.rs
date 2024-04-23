use crate::ember::gp::sink::TableEntry;
use crate::ember::gp::Address;
use crate::types::UintT;
use crate::{Error, Transport};
use std::future::Future;

pub trait SinkTable: Transport {
    /// Clear the entire sink table.
    fn clear_all(&self) -> impl Future<Output = Result<(), Error>>;

    /// Finds or allocates a sink entry.
    fn find_or_allocate_entry(&self, addr: Address) -> impl Future<Output = Result<u8, Error>>;

    /// Retrieves the sink table entry stored at the passed index.
    fn get_entry(&self, sink_index: u8) -> impl Future<Output = Result<TableEntry, Error>>;

    /// Initializes Sink Table.
    fn init(&self) -> impl Future<Output = Result<(), Error>>;

    /// Finds the index of the passed address in the gp table.
    fn lookup(&self, addr: Address) -> impl Future<Output = Result<u8, Error>>;

    /// Return number of active entries in sink table.
    fn number_of_active_entries(&self) -> impl Future<Output = Result<UintT, Error>>;

    /// Removes the sink table entry stored at the passed index.
    fn remove_entry(&self, sink_index: u8) -> impl Future<Output = Result<(), Error>>;

    /// Retrieves the sink table entry stored at the passed index.
    fn set_entry(
        &self,
        sink_index: u8,
        entry: TableEntry,
    ) -> impl Future<Output = Result<(), Error>>;

    /// Sets security frame counter in the sink table.
    fn set_security_frame_counter(
        &self,
        index: u8,
        sfc: u32,
    ) -> impl Future<Output = Result<(), Error>>;
}
