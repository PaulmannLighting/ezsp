mod response_handler;

use crate::ember;
use crate::error::Resolve;
use crate::frame::parameters::{
    add_endpoint, add_or_update_key_table_entry, add_transient_link_key,
    address_table_entry_is_active, aes_encrypt, aes_mmo_hash, binding_is_active,
    broadcast_network_key_switch, broadcast_next_network_key,
};
use crate::frame::{Control, Header};
use crate::transport::ashv2::response_handler::ResponseHandler;
use crate::transport::{Ezsp, Transport};
use crate::types::ByteSizedVec;
use crate::Error;
use ashv2::Host;
use le_stream::{FromLeBytes, ToLeBytes};
use serialport::SerialPort;
use std::fmt::Debug;

/// ASHv2 transport layer implementation.
#[derive(Debug)]
pub struct Ashv2<'a, S>
where
    S: SerialPort,
{
    host: Host<'a, S>,
    sequence: u8,
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
            sequence: 0,
            control,
        }
    }

    async fn communicate<R>(&mut self, payload: &[u8]) -> Result<R, Error>
    where
        R: Clone + Debug + FromLeBytes + ToLeBytes + Send + Sync + 'a,
    {
        self.host
            .communicate::<ResponseHandler<Control, u16, R>>(payload)
            .await
    }

    async fn communicate_legacy<R>(&mut self, payload: &[u8]) -> Result<R, Error>
    where
        R: Clone + Debug + FromLeBytes + ToLeBytes + Send + Sync + 'a,
    {
        self.host
            .communicate::<ResponseHandler<u8, u8, R>>(payload)
            .await
    }
}

impl<'a, S> Transport for Ashv2<'a, S>
where
    S: SerialPort,
{
    fn next_command<T>(&mut self, frame_id: u16, parameters: T) -> Vec<u8>
    where
        T: ToLeBytes,
    {
        let mut command = Vec::new();
        command.extend(Header::new(self.sequence, self.control, frame_id).to_le_bytes());
        self.sequence = self.sequence.checked_add(1).unwrap_or(0);
        command.extend_from_slice(&frame_id.to_le_bytes());
        command.extend(parameters.to_le_bytes());
        command
    }
}

impl<'a, S> Ezsp for Ashv2<'a, S>
where
    S: SerialPort,
{
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

    async fn add_or_update_key_table_entry(
        &mut self,
        address: ember::Eui64,
        link_key: bool,
        key_data: ember::key::Data,
    ) -> Result<(), Error> {
        let command = self.next_command(
            add_or_update_key_table_entry::ID,
            add_or_update_key_table_entry::Command::new(address, link_key, key_data),
        );
        self.communicate::<add_or_update_key_table_entry::Response>(command.as_slice())
            .await?
            .status()
            .resolve()
    }

    async fn add_transient_link_key(
        &mut self,
        partner: ember::Eui64,
        transient_key: ember::key::Data,
    ) -> Result<(), Error> {
        let command = self.next_command(
            add_transient_link_key::ID,
            add_transient_link_key::Command::new(partner, transient_key),
        );
        self.communicate::<add_transient_link_key::Response>(command.as_slice())
            .await?
            .status()
            .resolve()
    }

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

    async fn aes_encrypt(&mut self, plaintext: [u8; 16], key: [u8; 16]) -> Result<[u8; 16], Error> {
        let command = self.next_command(aes_encrypt::ID, aes_encrypt::Command::new(plaintext, key));
        self.communicate::<aes_encrypt::Response>(command.as_slice())
            .await
            .map(|response| response.ciphertext())
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

    async fn binding_is_active(&mut self, index: u8) -> Result<bool, Error> {
        let command = self.next_command(
            binding_is_active::ID,
            binding_is_active::Command::new(index),
        );
        self.communicate::<binding_is_active::Response>(command.as_slice())
            .await
            .map(|response| response.active())
    }

    async fn broadcast_network_key_switch(&mut self) -> Result<(), Error> {
        let command = self.next_command(broadcast_network_key_switch::ID, ());
        self.communicate::<broadcast_network_key_switch::Response>(command.as_slice())
            .await?
            .status()
            .resolve()
    }

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
}
