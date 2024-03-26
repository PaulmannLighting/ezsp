use std::fmt::Debug;
use std::sync::mpsc::Sender;
use std::sync::Arc;

use ashv2::Host;
use le_stream::{FromLeBytes, ToLeBytes};
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
use crate::frame::{Control, Header};
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
pub struct Ashv2<S>
where
    for<'s> S: SerialPort + 's,
{
    host: Host<S>,
    sequence: u8,
    control: Control,
}

impl<S> Ashv2<S>
where
    for<'s> S: SerialPort + 's,
{
    #[must_use]
    pub const fn new(host: Host<S>, control: Control) -> Self {
        Self {
            host,
            sequence: 0,
            control,
        }
    }

    /// Start the ASHv2 host.
    ///
    /// # Errors
    /// Returns an [`ashv2::Error`] if the host could not be started.
    pub fn start(&mut self, callback: Option<Sender<Arc<[u8]>>>) -> Result<(), ashv2::Error> {
        self.host.start(callback)
    }

    async fn communicate<R>(&mut self, payload: &[u8]) -> Result<R, Error>
    where
        for<'r> R: Clone + Debug + FromLeBytes + ToLeBytes + Send + Sync + 'r,
    {
        debug!("Sending payload: {payload:?}");
        self.host
            .communicate::<ResponseHandler<Control, u16, R>>(payload)
            .await
    }

    async fn communicate_legacy<R>(&mut self, payload: &[u8]) -> Result<R, Error>
    where
        for<'r> R: Clone + Debug + FromLeBytes + ToLeBytes + Send + Sync + 'r,
    {
        debug!("Sending legacy payload: {payload:?}");
        self.host
            .communicate::<ResponseHandler<u8, u8, R>>(payload)
            .await
    }
}

impl<S> Transport for Ashv2<S>
where
    for<'s> S: SerialPort + 's,
{
    fn next_command<I, T>(&mut self, frame_id: I, parameters: T) -> Vec<u8>
    where
        I: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
        T: ToLeBytes,
    {
        let mut command = Vec::new();
        let header = Header::new(self.sequence, self.control, frame_id);
        debug!("Header: {:?}", header.to_le_bytes().collect::<Vec<_>>());
        command.extend(header.to_le_bytes());
        self.sequence = self.sequence.checked_add(1).unwrap_or(0);
        command.extend(frame_id.to_le_bytes());
        command.extend(parameters.to_le_bytes());
        command
    }

    fn next_legacy_command<I, T>(&mut self, frame_id: I, parameters: T) -> Vec<u8>
    where
        I: Copy + Debug + Eq + PartialEq + FromLeBytes + ToLeBytes,
        T: ToLeBytes,
    {
        let mut command = Vec::new();
        let header = Header::new(self.sequence, self.control.low(), frame_id);
        debug!(
            "Legacy header: {:?}",
            header.to_le_bytes().collect::<Vec<_>>()
        );
        command.extend(header.to_le_bytes());
        self.sequence = self.sequence.checked_add(1).unwrap_or(0);
        command.extend(frame_id.to_le_bytes());
        command.extend(parameters.to_le_bytes());
        command
    }
}

impl<S> Binding for Ashv2<S>
where
    for<'s> S: SerialPort + 's,
{
    async fn clear_binding_table(&mut self) -> Result<(), Error> {
        let command = self.next_command(clear_binding_table::ID, ());
        self.communicate::<clear_binding_table::Response>(command.as_slice())
            .await?
            .status()
            .resolve()
    }

    async fn set_binding(&mut self, index: u8, value: TableEntry) -> Result<(), Error> {
        let command = self.next_command(set_binding::ID, set_binding::Command::new(index, value));
        self.communicate::<set_binding::Response>(command.as_slice())
            .await?
            .status()
            .resolve()
    }

    async fn get_binding(&mut self, index: u8) -> Result<TableEntry, Error> {
        let command = self.next_command(get_binding::ID, get_binding::Command::new(index));
        self.communicate::<get_binding::Response>(command.as_slice())
            .await
            .and_then(|response| response.status().resolve().map(|_| response.value()))
    }

    async fn delete_binding(&mut self, index: u8) -> Result<(), Error> {
        let command = self.next_command(delete_binding::ID, delete_binding::Command::new(index));
        self.communicate::<delete_binding::Response>(command.as_slice())
            .await?
            .status()
            .resolve()
    }

    async fn binding_is_active(&mut self, index: u8) -> Result<bool, Error> {
        let command = self.next_command(
            binding_is_active::ID,
            binding_is_active::Command::new(index),
        );
        self.communicate::<binding_is_active::Response>(command.as_slice())
            .await
            .map(|response| response.active())
    }

    async fn get_binding_remote_node_id(&mut self, index: u8) -> Result<NodeId, Error> {
        let command = self.next_command(
            get_binding_remote_node_id::ID,
            get_binding_remote_node_id::Command::new(index),
        );
        self.communicate::<get_binding_remote_node_id::Response>(command.as_slice())
            .await
            .map(|response| response.node_id())
    }

    async fn set_binding_remote_node_id(
        &mut self,
        index: u8,
        node_id: NodeId,
    ) -> Result<(), Error> {
        let command = self.next_command(
            set_binding_remote_node_id::ID,
            set_binding_remote_node_id::Command::new(index, node_id),
        );
        self.communicate::<()>(command.as_slice()).await
    }
}

impl<S> Bootloader for Ashv2<S>
where
    for<'s> S: SerialPort + 's,
{
    async fn aes_encrypt(&mut self, plaintext: [u8; 16], key: [u8; 16]) -> Result<[u8; 16], Error> {
        let command = self.next_command(aes_encrypt::ID, aes_encrypt::Command::new(plaintext, key));
        self.communicate::<aes_encrypt::Response>(command.as_slice())
            .await
            .map(|response| response.ciphertext())
    }
}

impl<S> CertificateBasedKeyExchange for Ashv2<S>
where
    for<'s> S: SerialPort + 's,
{
    async fn calculate_smacs(
        &mut self,
        am_initiator: bool,
        partner_certificate: ember::CertificateData,
        partner_ephemeral_public_key: ember::PublicKeyData,
    ) -> Result<(), Error> {
        let command = self.next_command(
            calculate_smacs::ID,
            calculate_smacs::Command::new(
                am_initiator,
                partner_certificate,
                partner_ephemeral_public_key,
            ),
        );
        self.communicate::<calculate_smacs::Response>(command.as_slice())
            .await?
            .status()
            .resolve()
    }
}

impl<S> Configuration for Ashv2<S>
where
    for<'s> S: SerialPort + 's,
{
    async fn version(
        &mut self,
        desired_protocol_version: u8,
    ) -> Result<version::Response<u16>, Error> {
        let command = self.next_command(
            u16::from(version::ID),
            version::Command::new(desired_protocol_version),
        );
        self.communicate::<version::Response<u16>>(command.as_slice())
            .await
    }

    async fn legacy_version(
        &mut self,
        desired_protocol_version: u8,
    ) -> Result<version::Response<u8>, Error> {
        let command =
            self.next_legacy_command(version::ID, version::Command::new(desired_protocol_version));
        self.communicate_legacy::<version::Response<u8>>(command.as_slice())
            .await
    }

    async fn get_configuration_value(&mut self, config_id: u8) -> Result<u16, Error> {
        todo!()
    }

    async fn set_configuration_value(&mut self, config_id: u8, value: u16) -> Result<(), Error> {
        todo!()
    }

    async fn read_attribute(
        &mut self,
        endpoint: u8,
        cluster: u16,
        attribute_id: u16,
        mask: u8,
        manufacturer_code: u16,
    ) -> Result<read_attribute::Response, Error> {
        todo!()
    }

    async fn write_attribute(
        &mut self,
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
        &mut self,
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_clusters: ByteSizedVec<u16>,
        output_clusters: ByteSizedVec<u16>,
    ) -> Result<(), Error> {
        let command = self.next_command(
            add_endpoint::ID,
            add_endpoint::Command::new(
                endpoint,
                profile_id,
                device_id,
                app_flags,
                input_clusters,
                output_clusters,
            ),
        );
        self.communicate::<add_endpoint::Response>(command.as_slice())
            .await?
            .status()
            .resolve()
    }

    async fn set_policy(&mut self, policy_id: u8, decision_id: u8) -> Result<(), Error> {
        todo!()
    }

    async fn get_policy(&mut self, policy_id: u8) -> Result<decision::Id, Error> {
        todo!()
    }

    async fn send_pan_id_update(&mut self, new_pan: PanId) -> Result<bool, Error> {
        todo!()
    }

    async fn get_value(&mut self, value_id: u8) -> Result<ByteSizedVec<u8>, Error> {
        todo!()
    }

    async fn get_extended_value(
        &mut self,
        value_id: u8,
        characteristics: u32,
    ) -> Result<ByteSizedVec<u8>, Error> {
        todo!()
    }

    async fn set_value(&mut self, value_id: u8, value: ByteSizedVec<u8>) -> Result<(), Error> {
        todo!()
    }

    async fn set_passive_ack_config(
        &mut self,
        config: u8,
        min_acks_needed: u8,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl<S> Messaging for Ashv2<S>
where
    for<'s> S: SerialPort + 's,
{
    async fn address_table_entry_is_active(
        &mut self,
        address_table_index: u8,
    ) -> Result<bool, Error> {
        let command = self.next_command(
            address_table_entry_is_active::ID,
            address_table_entry_is_active::Command::new(address_table_index),
        );
        self.communicate::<address_table_entry_is_active::Response>(command.as_slice())
            .await
            .map(|response| response.active())
    }
}

impl<S> TrustCenter for Ashv2<S>
where
    for<'s> S: SerialPort + 's,
{
    async fn broadcast_next_network_key(&mut self, key: ember::key::Data) -> Result<(), Error> {
        let command = self.next_command(
            broadcast_next_network_key::ID,
            broadcast_next_network_key::Command::new(key),
        );
        self.communicate::<broadcast_next_network_key::Response>(command.as_slice())
            .await?
            .status()
            .resolve()
    }

    async fn broadcast_network_key_switch(&mut self) -> Result<(), Error> {
        let command = self.next_command(broadcast_network_key_switch::ID, ());
        self.communicate::<broadcast_network_key_switch::Response>(command.as_slice())
            .await?
            .status()
            .resolve()
    }

    async fn aes_mmo_hash(
        &mut self,
        context: ember::aes::MmoHashContext,
        finalize: bool,
        data: ByteSizedVec<u8>,
    ) -> Result<ember::aes::MmoHashContext, Error> {
        let command = self.next_command(
            aes_mmo_hash::ID,
            aes_mmo_hash::Command::new(context, finalize, data),
        );
        let result = self
            .communicate::<aes_mmo_hash::Response>(command.as_slice())
            .await?;
        result.status().resolve().map(|()| result.return_context())
    }
}
