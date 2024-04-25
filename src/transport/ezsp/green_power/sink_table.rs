use std::future::Future;

use crate::ember::gp::sink::TableEntry;
use crate::ember::gp::Address;
use crate::frame::parameters::green_power::sink_table::{clear_all, find_or_allocate_entry};
use crate::types::UintT;
use crate::{Error, Transport};

pub trait SinkTable {
    /// Clear the entire sink table.
    fn clear_all(&self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Finds or allocates a sink entry.
    fn find_or_allocate_entry(
        &self,
        addr: Address,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Retrieves the sink table entry stored at the passed index.
    fn get_entry(&self, sink_index: u8) -> impl Future<Output = Result<TableEntry, Error>> + Send;

    /// Initializes Sink Table.
    fn init(&self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Finds the index of the passed address in the gp table.
    fn lookup(&self, addr: Address) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Return number of active entries in sink table.
    fn number_of_active_entries(&self) -> impl Future<Output = Result<UintT, Error>> + Send;

    /// Removes the sink table entry stored at the passed index.
    fn remove_entry(&self, sink_index: u8) -> impl Future<Output = Result<(), Error>> + Send;

    /// Retrieves the sink table entry stored at the passed index.
    fn set_entry(
        &self,
        sink_index: u8,
        entry: TableEntry,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sets security frame counter in the sink table.
    fn set_security_frame_counter(
        &self,
        index: u8,
        sfc: u32,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> SinkTable for T
where
    T: Transport,
{
    async fn clear_all(&self) -> Result<(), Error> {
        self.communicate::<_, clear_all::Response>(clear_all::Command)
            .await
            .map(drop)
    }

    async fn find_or_allocate_entry(&self, addr: Address) -> Result<u8, Error> {
        self.communicate::<_, find_or_allocate_entry::Response>(
            find_or_allocate_entry::Command::new(addr),
        )
        .await
        .map(|response| response.index())
    }

    async fn get_entry(&self, sink_index: u8) -> Result<TableEntry, Error> {
        todo!()
    }

    async fn init(&self) -> Result<(), Error> {
        todo!()
    }

    async fn lookup(&self, addr: Address) -> Result<u8, Error> {
        todo!()
    }

    async fn number_of_active_entries(&self) -> Result<UintT, Error> {
        todo!()
    }

    async fn remove_entry(&self, sink_index: u8) -> Result<(), Error> {
        todo!()
    }

    async fn set_entry(&self, sink_index: u8, entry: TableEntry) -> Result<(), Error> {
        todo!()
    }

    async fn set_security_frame_counter(&self, index: u8, sfc: u32) -> Result<(), Error> {
        todo!()
    }
}
