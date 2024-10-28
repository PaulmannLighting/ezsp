use std::future::Future;

use crate::ember::aes::MmoHashContext;
use crate::ember::key::Data;
use crate::ember::{Eui64, NodeId};
use crate::frame::parameters::trust_center::{
    aes_mmo_hash, broadcast_network_key_switch, broadcast_next_network_key, remove_device,
    unicast_nwk_key_update,
};
use crate::types::ByteSizedVec;
use crate::{Error, Transport};

/// The `TrustCenter` trait provides an interface for the Trust Center features.
pub trait TrustCenter {
    /// This routine processes the passed chunk of data and updates the hash context based on it.
    ///
    /// If the `finalize` parameter is not set, then the length of the data passed in must be a
    /// multiple of 16.
    ///
    /// If the `finalize` parameter is set then the length can be any value up 1-16,
    /// and the final hash value will be calculated.
    fn aes_mmo_hash(
        &mut self,
        context: MmoHashContext,
        finalize: bool,
        data: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<MmoHashContext, Error>> + Send;

    /// This function broadcasts a switch key message to tell all nodes to change to the
    /// sequence number of the previously sent Alternate Encryption Key.
    fn broadcast_network_key_switch(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// This function broadcasts a new encryption key,
    /// but does not tell the nodes in the network to start using it.
    ///
    /// To tell nodes to switch to the new key, use [`broadcast_network_key_switch()`](Self::broadcast_network_key_switch).
    /// This is only valid for the Trust Center/Coordinator.
    ///
    /// It is up to the application to determine how quickly
    /// to send the Switch Key after sending the alternate encryption key.
    fn broadcast_next_network_key(
        &mut self,
        key: Data,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// This command sends an APS remove device using APS encryption to the destination indicating
    /// either to remove itself from the network, or one of its children.
    fn remove_device(
        &mut self,
        dest_short: NodeId,
        dest_long: Eui64,
        target_long: Eui64,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// This command will send a unicast transport key message with a new NWK key
    /// to the specified device.
    ///
    /// APS encryption using the device's existing link key will be used.
    fn unicast_nwk_key_update(
        &mut self,
        dest_short: NodeId,
        dest_long: Eui64,
        key: Data,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> TrustCenter for T
where
    T: Transport,
{
    async fn aes_mmo_hash(
        &mut self,
        context: MmoHashContext,
        finalize: bool,
        data: ByteSizedVec<u8>,
    ) -> Result<MmoHashContext, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, aes_mmo_hash::Response>(aes_mmo_hash::Command::new(
            context, finalize, data,
        ))
        .await?
        .try_into()
    }

    async fn broadcast_network_key_switch(&mut self) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, broadcast_network_key_switch::Response>(
            broadcast_network_key_switch::Command,
        )
        .await?
        .try_into()
    }

    async fn broadcast_next_network_key(&mut self, key: Data) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, broadcast_next_network_key::Response>(
            broadcast_next_network_key::Command::new(key),
        )
        .await?
        .try_into()
    }

    async fn remove_device(
        &mut self,
        dest_short: NodeId,
        dest_long: Eui64,
        target_long: Eui64,
    ) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, remove_device::Response>(remove_device::Command::new(
            dest_short,
            dest_long,
            target_long,
        ))
        .await?
        .try_into()
    }

    async fn unicast_nwk_key_update(
        &mut self,
        dest_short: NodeId,
        dest_long: Eui64,
        key: Data,
    ) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, unicast_nwk_key_update::Response>(
            unicast_nwk_key_update::Command::new(dest_short, dest_long, key),
        )
        .await?
        .try_into()
    }
}
