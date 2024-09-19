use std::future::Future;

use siliconlabs;
use siliconlabs::zigbee::security::{ManContext, ManKey};

use crate::ember::Eui64;
use crate::error::Resolve;
use crate::frame::parameters::security::{
    check_key_context, clear_key_table, clear_transient_link_keys, erase_key_table_entry,
    export_key, export_link_key_by_eui, export_link_key_by_index, export_transient_key,
};
use crate::{Error, Transport};

pub trait Security {
    /// Check whether a key context can be used to load a valid key.
    fn check_key_context(
        &self,
        context: ManContext,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// This function clears the key table of the current network.
    fn clear_key_table(&self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Clear all the transient link keys from RAM.
    fn clear_transient_link_keys(&self) -> impl Future<Output = Result<(), Error>> + Send;

    /// This function erases the data in the key table entry at the specified index.
    ///If the index is invalid, false is returned.
    fn erase_key_table_entry(&self, index: u8) -> impl Future<Output = Result<(), Error>> + Send;

    /// Exports a key from security manager based on passed context.
    fn export_key(
        &self,
        man_context: ManContext,
    ) -> impl Future<Output = Result<ManKey, Error>> + Send;

    /// Export the link key associated with the given EUI from the key table.
    fn export_link_key_by_eui(
        &self,
        eui: Eui64,
    ) -> impl Future<Output = Result<export_link_key_by_eui::Response, Error>> + Send;

    /// Export the link key at given index from the key table.
    fn export_link_key_by_index(
        &self,
        index: u8,
    ) -> impl Future<Output = Result<export_link_key_by_index::Response, Error>> + Send;

    /// Export a transient link key associated with a given EUI64.
    fn export_transient_key_by_eui(
        &self,
        eui: Eui64,
    ) -> impl Future<Output = Result<export_transient_key::Payload, Error>> + Send;

    /// Export a transient link key from a given table index.
    fn export_transient_key_by_index(
        &self,
        index: u8,
    ) -> impl Future<Output = Result<export_transient_key::Payload, Error>> + Send;
}

impl<T> Security for T
where
    T: Transport,
{
    async fn check_key_context(&self, context: ManContext) -> Result<(), Error> {
        self.communicate::<_, check_key_context::Response>(check_key_context::Command::new(context))
            .await?
            .resolve()
    }

    async fn clear_key_table(&self) -> Result<(), Error> {
        self.communicate::<_, clear_key_table::Response>(clear_key_table::Command)
            .await?
            .resolve()
    }

    async fn clear_transient_link_keys(&self) -> Result<(), Error> {
        self.communicate::<_, clear_transient_link_keys::Response>(
            clear_transient_link_keys::Command,
        )
        .await
        .map(drop)
    }

    async fn erase_key_table_entry(&self, index: u8) -> Result<(), Error> {
        self.communicate::<_, erase_key_table_entry::Response>(erase_key_table_entry::Command::new(
            index,
        ))
        .await
        .map(drop)
    }

    async fn export_key(&self, man_context: ManContext) -> Result<ManKey, Error> {
        self.communicate::<_, export_key::Response>(export_key::Command::new(man_context))
            .await?
            .resolve()
    }

    async fn export_link_key_by_eui(
        &self,
        eui: Eui64,
    ) -> Result<export_link_key_by_eui::Response, Error> {
        self.communicate::<_, export_link_key_by_eui::Response>(
            export_link_key_by_eui::Command::new(eui),
        )
        .await?
        .resolve()
    }

    async fn export_link_key_by_index(
        &self,
        index: u8,
    ) -> Result<export_link_key_by_index::Response, Error> {
        self.communicate::<_, export_link_key_by_index::Response>(
            export_link_key_by_index::Command::new(index),
        )
        .await?
        .resolve()
    }

    async fn export_transient_key_by_eui(
        &self,
        eui: Eui64,
    ) -> Result<export_transient_key::Payload, Error> {
        self.communicate::<_, export_transient_key::by_eui::Response>(
            export_transient_key::by_eui::Command::new(eui),
        )
        .await?
        .resolve()
    }

    async fn export_transient_key_by_index(
        &self,
        index: u8,
    ) -> Result<export_transient_key::Payload, Error> {
        self.communicate::<_, export_transient_key::by_index::Response>(
            export_transient_key::by_index::Command::new(index),
        )
        .await?
        .resolve()
    }
}
