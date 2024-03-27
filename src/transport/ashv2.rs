use std::fmt::Debug;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::mpsc::Sender;
use std::sync::Arc;

use ashv2::Host;
use le_stream::ToLeBytes;
use log::debug;
use serialport::SerialPort;

use crate::ember;
use crate::ember::binding::TableEntry;
use crate::ember::{NodeId, PanId};
use crate::error::Resolve;
use crate::ezsp::decision;
use crate::frame::parameters::{
    add_endpoint, address_table_entry_is_active, aes_encrypt, aes_mmo_hash, binding_is_active,
    broadcast_network_key_switch, broadcast_next_network_key, calculate_smacs, clear_binding_table,
    delete_binding, get_binding, get_binding_remote_node_id, read_attribute, set_binding,
    set_binding_remote_node_id, version,
};
use crate::frame::{Control, Header, Parameter};
use crate::transport::ashv2::response_handler::ResponseHandler;
use crate::transport::ezsp::{
    Binding, Bootloader, CertificateBasedKeyExchange, Configuration, Messaging, TrustCenter,
};
use crate::transport::{Ezsp, Transport};
use crate::types::ByteSizedVec;
use crate::Error;

mod response_handler;

/// ASHv2 transport layer implementation.
#[derive(Debug)]
pub struct Ashv2<'a, S>
where
    S: SerialPort + 'a,
{
    host: Host<'a, S>,
    sequence: AtomicU8,
    control: Control,
}

impl<'a, S> Ashv2<'a, S>
where
    S: SerialPort,
{
    #[must_use]
    pub const fn new(host: Host<'a, S>, control: Control) -> Self {
        Self {
            host,
            sequence: AtomicU8::new(0),
            control,
        }
    }

    /// Start the ASHv2 host.
    ///
    /// # Errors
    /// Returns an [`ashv2::Error`] if the host could not be started.
    pub fn start(&mut self, callback: Option<Sender<Arc<[u8]>>>) -> Result<(), ashv2::Error>
    where
        Self: 'static,
    {
        self.host.start(callback)
    }
}

impl<'a, S> Transport for Ashv2<'a, S>
where
    Self: 'static,
    S: SerialPort,
{
    fn next_header<R>(&self) -> Header<R::Id>
    where
        R: Parameter,
    {
        let sequence = self.sequence.load(SeqCst);
        let header = Header::new(sequence, self.control.into(), R::ID);
        debug!("Header: {:?}", header.to_le_bytes().collect::<Vec<_>>());
        self.sequence
            .store(sequence.checked_add(1).unwrap_or(0), SeqCst);
        header
    }

    async fn communicate<R>(&self, command: impl Parameter) -> Result<R, Error>
    where
        R: Clone + Debug + Parameter + Send + Sync + 'a,
    {
        let mut payload = Vec::new();
        payload.extend(self.next_header::<R>().to_le_bytes());
        payload.extend(command.to_le_bytes());
        debug!("Sending payload: {payload:?}");
        self.host.communicate::<ResponseHandler<R>>(&payload).await
    }
}

impl<S> Binding for Ashv2<'_, S>
where
    Self: 'static,
    S: SerialPort,
{
    async fn clear_binding_table(&self) -> Result<(), Error> {
        self.communicate::<clear_binding_table::Response>(clear_binding_table::Command)
            .await?
            .status()
            .resolve()
    }

    async fn set_binding(&self, index: u8, value: TableEntry) -> Result<(), Error> {
        self.communicate::<set_binding::Response>(set_binding::Command::new(index, value))
            .await?
            .status()
            .resolve()
    }

    async fn get_binding(&self, index: u8) -> Result<TableEntry, Error> {
        self.communicate::<get_binding::Response>(get_binding::Command::new(index))
            .await
            .and_then(|response| response.status().resolve().map(|_| response.value()))
    }

    async fn delete_binding(&self, index: u8) -> Result<(), Error> {
        self.communicate::<delete_binding::Response>(delete_binding::Command::new(index))
            .await?
            .status()
            .resolve()
    }

    async fn binding_is_active(&self, index: u8) -> Result<bool, Error> {
        self.communicate::<binding_is_active::Response>(binding_is_active::Command::new(index))
            .await
            .map(|response| response.active())
    }

    async fn get_binding_remote_node_id(&self, index: u8) -> Result<NodeId, Error> {
        self.communicate::<get_binding_remote_node_id::Response>(
            get_binding_remote_node_id::Command::new(index),
        )
        .await
        .map(|response| response.node_id())
    }

    async fn set_binding_remote_node_id(&self, index: u8, node_id: NodeId) -> Result<(), Error> {
        self.communicate::<set_binding_remote_node_id::Response>(
            set_binding_remote_node_id::Command::new(index, node_id),
        )
        .await
        .map(|_| ())
    }
}

impl<S> Bootloader for Ashv2<'_, S>
where
    Self: 'static,
    S: SerialPort,
{
    async fn aes_encrypt(&self, plaintext: [u8; 16], key: [u8; 16]) -> Result<[u8; 16], Error> {
        self.communicate::<aes_encrypt::Response>(aes_encrypt::Command::new(plaintext, key))
            .await
            .map(|response| response.ciphertext())
    }
}

impl<S> CertificateBasedKeyExchange for Ashv2<'_, S>
where
    Self: 'static,
    S: SerialPort,
{
    async fn calculate_smacs(
        &self,
        am_initiator: bool,
        partner_certificate: ember::CertificateData,
        partner_ephemeral_public_key: ember::PublicKeyData,
    ) -> Result<(), Error> {
        self.communicate::<calculate_smacs::Response>(calculate_smacs::Command::new(
            am_initiator,
            partner_certificate,
            partner_ephemeral_public_key,
        ))
        .await?
        .status()
        .resolve()
    }
}

impl<S> Configuration for Ashv2<'_, S>
where
    Self: 'static,
    S: SerialPort,
{
    async fn version(&self, desired_protocol_version: u8) -> Result<version::Response, Error> {
        self.communicate::<version::Response>(version::Command::new(desired_protocol_version))
            .await
    }

    async fn legacy_version(
        &self,
        desired_protocol_version: u8,
    ) -> Result<version::Response, Error> {
        self.communicate::<version::LegacyResponse>(version::LegacyCommand::from(
            version::Command::new(desired_protocol_version),
        ))
        .await
        .map(Into::into)
    }

    async fn get_configuration_value(&self, config_id: u8) -> Result<u16, Error> {
        todo!()
    }

    async fn set_configuration_value(&self, config_id: u8, value: u16) -> Result<(), Error> {
        todo!()
    }

    async fn read_attribute(
        &self,
        endpoint: u8,
        cluster: u16,
        attribute_id: u16,
        mask: u8,
        manufacturer_code: u16,
    ) -> Result<read_attribute::Response, Error> {
        todo!()
    }

    async fn write_attribute(
        &self,
        endpoint: u8,
        cluster: u16,
        attribute_id: u16,
        mask: u8,
        manufacturer_code: u16,
        just_test: bool,
        data_type: u8,
        data: ByteSizedVec<u8>,
    ) -> Result<(), Error> {
        todo!()
    }

    async fn add_endpoint(
        &self,
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_clusters: ByteSizedVec<u16>,
        output_clusters: ByteSizedVec<u16>,
    ) -> Result<(), Error> {
        self.communicate::<add_endpoint::Response>(add_endpoint::Command::new(
            endpoint,
            profile_id,
            device_id,
            app_flags,
            input_clusters,
            output_clusters,
        ))
        .await?
        .status()
        .resolve()
    }

    async fn set_policy(&self, policy_id: u8, decision_id: u8) -> Result<(), Error> {
        todo!()
    }

    async fn get_policy(&self, policy_id: u8) -> Result<decision::Id, Error> {
        todo!()
    }

    async fn send_pan_id_update(&self, new_pan: PanId) -> Result<bool, Error> {
        todo!()
    }

    async fn get_value(&self, value_id: u8) -> Result<ByteSizedVec<u8>, Error> {
        todo!()
    }

    async fn get_extended_value(
        &self,
        value_id: u8,
        characteristics: u32,
    ) -> Result<ByteSizedVec<u8>, Error> {
        todo!()
    }

    async fn set_value(&self, value_id: u8, value: ByteSizedVec<u8>) -> Result<(), Error> {
        todo!()
    }

    async fn set_passive_ack_config(&self, config: u8, min_acks_needed: u8) -> Result<(), Error> {
        todo!()
    }
}

impl<S> Messaging for Ashv2<'_, S>
where
    Self: 'static,
    S: SerialPort,
{
    async fn address_table_entry_is_active(&self, address_table_index: u8) -> Result<bool, Error> {
        self.communicate::<address_table_entry_is_active::Response>(
            address_table_entry_is_active::Command::new(address_table_index),
        )
        .await
        .map(|response| response.active())
    }
}

impl<S> TrustCenter for Ashv2<'_, S>
where
    Self: 'static,
    S: SerialPort,
{
    async fn broadcast_next_network_key(&self, key: ember::key::Data) -> Result<(), Error> {
        self.communicate::<broadcast_next_network_key::Response>(
            broadcast_next_network_key::Command::new(key),
        )
        .await?
        .status()
        .resolve()
    }

    async fn broadcast_network_key_switch(&self) -> Result<(), Error> {
        self.communicate::<broadcast_network_key_switch::Response>(
            broadcast_network_key_switch::Command,
        )
        .await?
        .status()
        .resolve()
    }

    async fn aes_mmo_hash(
        &self,
        context: ember::aes::MmoHashContext,
        finalize: bool,
        data: ByteSizedVec<u8>,
    ) -> Result<ember::aes::MmoHashContext, Error> {
        let result = self
            .communicate::<aes_mmo_hash::Response>(aes_mmo_hash::Command::new(
                context, finalize, data,
            ))
            .await?;
        result.status().resolve().map(|()| result.return_context())
    }
}
