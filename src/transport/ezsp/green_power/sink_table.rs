use std::future::Future;

use crate::ember::gp::sink::TableEntry;
use crate::ember::gp::Address;
use crate::error::Error;
use crate::frame::parameters::green_power::sink_table::{
    clear_all, find_or_allocate_entry, get_entry, init, lookup, number_of_active_entries,
    remove_entry, set_entry, set_security_frame_counter,
};
use crate::transport::Transport;
use crate::types::UintT;

/// The `SinkTable` trait provides an interface for the sink table.
pub trait SinkTable {
    /// Clear the entire sink table.
    fn clear_all(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Finds or allocates a sink entry.
    fn find_or_allocate_entry(
        &mut self,
        addr: Address,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Retrieves the sink table entry stored at the passed index.
    fn get_entry(
        &mut self,
        sink_index: u8,
    ) -> impl Future<Output = Result<TableEntry, Error>> + Send;

    /// Initializes Sink Table.
    fn init(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Finds the index of the passed address in the gp table.
    fn lookup(&mut self, addr: Address) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Return number of active entries in sink table.
    fn number_of_active_entries(&mut self) -> impl Future<Output = Result<UintT, Error>> + Send;

    /// Removes the sink table entry stored at the passed index.
    fn remove_entry(&mut self, sink_index: u8) -> impl Future<Output = Result<(), Error>> + Send;

    /// Retrieves the sink table entry stored at the passed index.
    fn set_entry(
        &mut self,
        sink_index: u8,
        entry: TableEntry,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sets security frame counter in the sink table.
    fn set_security_frame_counter(
        &mut self,
        index: u8,
        sfc: u32,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> SinkTable for T
where
    T: Transport,
{
    async fn clear_all(&mut self) -> Result<(), Error> {
        Ok(clear_all::Response::try_from(self.communicate(clear_all::Command).await?).map(drop)?)
    }

    async fn find_or_allocate_entry(&mut self, addr: Address) -> Result<u8, Error> {
        Ok(find_or_allocate_entry::Response::try_from(
            self.communicate(find_or_allocate_entry::Command::new(addr))
                .await?,
        )
        .map(|response| response.index())?)
    }

    async fn get_entry(&mut self, sink_index: u8) -> Result<TableEntry, Error> {
        get_entry::Response::try_from(
            self.communicate(get_entry::Command::new(sink_index))
                .await?,
        )?
        .try_into()
    }

    async fn init(&mut self) -> Result<(), Error> {
        Ok(init::Response::try_from(self.communicate(init::Command).await?).map(drop)?)
    }

    async fn lookup(&mut self, addr: Address) -> Result<u8, Error> {
        Ok(
            lookup::Response::try_from(self.communicate(lookup::Command::new(addr)).await?)
                .map(|response| response.index())?,
        )
    }

    async fn number_of_active_entries(&mut self) -> Result<UintT, Error> {
        Ok(number_of_active_entries::Response::try_from(
            self.communicate(number_of_active_entries::Command).await?,
        )
        .map(|response| response.number_of_entries())?)
    }

    async fn remove_entry(&mut self, sink_index: u8) -> Result<(), Error> {
        Ok(remove_entry::Response::try_from(
            self.communicate(remove_entry::Command::new(sink_index))
                .await?,
        )
        .map(drop)?)
    }

    async fn set_entry(&mut self, sink_index: u8, entry: TableEntry) -> Result<(), Error> {
        set_entry::Response::try_from(
            self.communicate(set_entry::Command::new(sink_index, entry))
                .await?,
        )?
        .try_into()
    }

    async fn set_security_frame_counter(&mut self, index: u8, sfc: u32) -> Result<(), Error> {
        Ok(set_security_frame_counter::Response::try_from(
            self.communicate(set_security_frame_counter::Command::new(index, sfc))
                .await?,
        )
        .map(drop)?)
    }
}
