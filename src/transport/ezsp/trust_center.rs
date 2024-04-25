use std::future::Future;

use crate::ember::aes::MmoHashContext;
use crate::ember::key::Data;
use crate::error::Resolve;
use crate::frame::parameters::trust_center::{
    aes_mmo_hash, broadcast_network_key_switch, broadcast_next_network_key,
};
use crate::types::ByteSizedVec;
use crate::{Error, Transport};

pub trait TrustCenter {
    /// This function broadcasts a new encryption key,
    /// but does not tell the nodes in the network to start using it.
    /// To tell nodes to switch to the new key, use [`broadcast_network_key_switch()`](Self::broadcast_network_key_switch).
    /// This is only valid for the Trust Center/Coordinator.
    /// It is up to the application to determine how quickly
    /// to send the Switch Key after sending the alternate encryption key.
    fn broadcast_next_network_key(&self, key: Data) -> impl Future<Output = Result<(), Error>>;

    /// This function broadcasts a switch key message to tell all nodes to change to the
    /// sequence number of the previously sent Alternate Encryption Key.
    fn broadcast_network_key_switch(&self) -> impl Future<Output = Result<(), Error>> + Send;

    /// This routine processes the passed chunk of data and updates the hash context based on it.
    /// If the `finalize` parameter is not set, then the length of the data passed in must be a
    /// multiple of 16.
    /// If the `finalize` parameter is set then the length can be any value up 1-16,
    /// and the final hash value will be calculated.
    fn aes_mmo_hash(
        &self,
        context: MmoHashContext,
        finalize: bool,
        data: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<MmoHashContext, Error>> + Send;
}

impl<T> TrustCenter for T
where
    T: Transport,
{
    async fn broadcast_next_network_key(&self, key: Data) -> Result<(), Error> {
        self.communicate::<_, broadcast_next_network_key::Response>(
            broadcast_next_network_key::Command::new(key),
        )
        .await?
        .status()
        .resolve()
    }

    async fn broadcast_network_key_switch(&self) -> Result<(), Error> {
        self.communicate::<_, broadcast_network_key_switch::Response>(
            broadcast_network_key_switch::Command,
        )
        .await?
        .status()
        .resolve()
    }

    async fn aes_mmo_hash(
        &self,
        context: MmoHashContext,
        finalize: bool,
        data: ByteSizedVec<u8>,
    ) -> Result<MmoHashContext, Error> {
        self.communicate::<_, aes_mmo_hash::Response>(aes_mmo_hash::Command::new(
            context, finalize, data,
        ))
        .await?
        .into()
    }
}
