use std::future::Future;

use siliconlabs::zigbee::security::man;

use crate::ember::key::{Struct, Type};
use crate::ember::{security, Eui64, NodeId};
use crate::error::Error;
use crate::frame::parameters::security::{
    check_key_context, clear_key_table, clear_transient_link_keys, erase_key_table_entry,
    export_key, export_link_key_by_eui, export_link_key_by_index, export_transient_key,
    find_key_table_entry, get_aps_key_info, get_current_security_state, get_network_key_info,
    import_key, import_link_key, import_transient_key, request_link_key,
    send_trust_center_link_key, set_initial_security_state, update_tc_link_key,
};
use crate::parameters::security::get_key;
use crate::transport::Transport;

/// The `Security` trait provides an interface for the security features.
pub trait Security {
    /// Check whether a key context can be used to load a valid key.
    fn check_key_context(
        &mut self,
        context: man::Context<Eui64>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// This function clears the key table of the current network.
    fn clear_key_table(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Clear all the transient link keys from RAM.
    fn clear_transient_link_keys(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// This function erases the data in the key table entry at the specified index.
    ///If the index is invalid, false is returned.
    fn erase_key_table_entry(
        &mut self,
        index: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Exports a key from security manager based on passed context.
    fn export_key(
        &mut self,
        man_context: man::Context<Eui64>,
    ) -> impl Future<Output = Result<man::Key, Error>> + Send;

    /// Export the link key associated with the given EUI from the key table.
    fn export_link_key_by_eui(
        &mut self,
        eui: Eui64,
    ) -> impl Future<Output = Result<export_link_key_by_eui::Payload, Error>> + Send;

    /// Export the link key at given index from the key table.
    fn export_link_key_by_index(
        &mut self,
        index: u8,
    ) -> impl Future<Output = Result<export_link_key_by_index::Payload, Error>> + Send;

    /// Export a transient link key associated with a given EUI64.
    fn export_transient_key_by_eui(
        &mut self,
        eui: Eui64,
    ) -> impl Future<Output = Result<export_transient_key::TransientKey, Error>> + Send;

    /// Export a transient link key from a given table index.
    fn export_transient_key_by_index(
        &mut self,
        index: u8,
    ) -> impl Future<Output = Result<export_transient_key::TransientKey, Error>> + Send;

    /// This function searches through the Key Table and tries to find the entry
    /// that matches the passed search criteria.
    fn find_key_table_entry(
        &mut self,
        address: Eui64,
        link_key: bool,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Retrieve metadata about an APS link key. Does not retrieve contents.
    fn get_aps_key_info(
        &mut self,
        context_in: man::Context<Eui64>,
    ) -> impl Future<Output = Result<get_aps_key_info::KeyInfo, Error>> + Send;

    /// Gets the current security state that is being used by a device that is joined in the network.
    fn get_current_security_state(
        &mut self,
    ) -> impl Future<Output = Result<security::current::State, Error>> + Send;

    /// Retrieve a key from the key table.
    ///
    /// # Deprecated
    ///
    /// This command has been removed in revision 5.1 of the `EZSP` specification.
    #[deprecated]
    fn get_key(&mut self, key: Type) -> impl Future<Output = Result<Struct, Error>> + Send;

    /// Retrieve information about the current and alternate network key, excluding their contents.
    fn get_network_key_info(
        &mut self,
    ) -> impl Future<Output = Result<man::NetworkKeyInfo, Error>> + Send;

    /// Imports a key into security manager based on passed context.
    fn import_key(
        &mut self,
        context: man::Context<Eui64>,
        key: man::Key,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Import an application link key into the key table.
    fn import_link_key(
        &mut self,
        index: u8,
        address: Eui64,
        plaintext_key: man::Key,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Import a transient link key.
    fn import_transient_key(
        &mut self,
        context: man::Context<Eui64>,
        eui64: Eui64,
        plaintext_key: man::Key,
        flags: man::Flags,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// A function to request a Link Key from the Trust Center with another device on the Network
    /// (which could be the Trust Center).
    ///
    /// A Link Key with the Trust Center is possible but the requesting device cannot be the Trust Center.
    /// Link Keys are optional in Zigbee Standard Security and thus the stack cannot know whether the other device supports them.
    ///
    /// If `EMBER_REQUEST_KEY_TIMEOUT` is non-zero on the Trust Center and the partner device is not the Trust Center,
    /// both devices must request keys with their partner device within the time period.
    ///
    /// The Trust Center only supports one outstanding key request at a time and therefore will ignore other requests.
    /// If the timeout is zero then the Trust Center will immediately respond and not wait for the second request.
    /// The Trust Center will always immediately respond to requests for a Link Key with it.
    ///
    /// Sleepy devices should poll at a higher rate until a response is received or the request times out.
    /// The success or failure of the request is returned via `ezspZigbeeKeyEstablishmentHandler(...)`.
    fn request_link_key(
        &mut self,
        partner: Eui64,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// This function sends an APS `TransportKey` command containing the current trust center link key.
    ///
    /// The node to which the command is sent is specified via the short and long address arguments.
    fn send_trust_center_link_key(
        &mut self,
        destination_node_id: NodeId,
        destination_eui64: Eui64,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sets the security state that will be used by the device when it forms or joins the network.
    ///
    /// This call should not be used when restoring saved network state via networkInit as this will
    /// result in a loss of security data and will cause communication problems when the device
    /// re-enters the network.
    fn set_initial_security_state(
        &mut self,
        state: security::initial::State,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Requests a new link key from the Trust Center.
    ///
    /// This function starts by sending a Node Descriptor request to the Trust Center to verify its
    /// R21+ stack version compliance. A Request Key message will then be sent, followed by a
    /// Verify Key Confirm message.
    fn update_tc_link_key(
        &mut self,
        max_attempts: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> Security for T
where
    T: Transport,
{
    async fn check_key_context(&mut self, context: man::Context<Eui64>) -> Result<(), Error> {
        self.communicate::<_, check_key_context::Response>(check_key_context::Command::new(context))
            .await?
            .try_into()
    }

    async fn clear_key_table(&mut self) -> Result<(), Error> {
        self.communicate::<_, clear_key_table::Response>(clear_key_table::Command)
            .await?
            .try_into()
    }

    async fn clear_transient_link_keys(&mut self) -> Result<(), Error> {
        self.communicate::<_, clear_transient_link_keys::Response>(
            clear_transient_link_keys::Command,
        )
        .await
        .map(drop)
    }

    async fn erase_key_table_entry(&mut self, index: u8) -> Result<(), Error> {
        self.communicate::<_, erase_key_table_entry::Response>(erase_key_table_entry::Command::new(
            index,
        ))
        .await
        .map(drop)
    }

    async fn export_key(&mut self, man_context: man::Context<Eui64>) -> Result<man::Key, Error> {
        self.communicate::<_, export_key::Response>(export_key::Command::new(man_context))
            .await?
            .try_into()
    }

    async fn export_link_key_by_eui(
        &mut self,
        eui: Eui64,
    ) -> Result<export_link_key_by_eui::Payload, Error> {
        self.communicate::<_, export_link_key_by_eui::Response>(
            export_link_key_by_eui::Command::new(eui),
        )
        .await?
        .try_into()
    }

    async fn export_link_key_by_index(
        &mut self,
        index: u8,
    ) -> Result<export_link_key_by_index::Payload, Error> {
        self.communicate::<_, export_link_key_by_index::Response>(
            export_link_key_by_index::Command::new(index),
        )
        .await?
        .try_into()
    }

    async fn export_transient_key_by_eui(
        &mut self,
        eui: Eui64,
    ) -> Result<export_transient_key::TransientKey, Error> {
        self.communicate::<_, export_transient_key::by_eui::Response>(
            export_transient_key::by_eui::Command::new(eui),
        )
        .await?
        .try_into()
    }

    async fn export_transient_key_by_index(
        &mut self,
        index: u8,
    ) -> Result<export_transient_key::TransientKey, Error> {
        self.communicate::<_, export_transient_key::by_index::Response>(
            export_transient_key::by_index::Command::new(index),
        )
        .await?
        .try_into()
    }

    async fn find_key_table_entry(&mut self, address: Eui64, link_key: bool) -> Result<u8, Error> {
        self.communicate::<_, find_key_table_entry::Response>(find_key_table_entry::Command::new(
            address, link_key,
        ))
        .await
        .map(Into::into)
    }

    async fn get_aps_key_info(
        &mut self,
        context_in: man::Context<Eui64>,
    ) -> Result<get_aps_key_info::KeyInfo, Error> {
        self.communicate::<_, get_aps_key_info::Response>(get_aps_key_info::Command::new(
            context_in,
        ))
        .await?
        .try_into()
    }

    async fn get_current_security_state(&mut self) -> Result<security::current::State, Error> {
        self.communicate::<_, get_current_security_state::Response>(
            get_current_security_state::Command,
        )
        .await?
        .try_into()
    }

    async fn get_key(&mut self, key: Type) -> Result<Struct, Error> {
        self.communicate::<_, get_key::Response>(get_key::Command::new(key))
            .await?
            .try_into()
    }

    async fn get_network_key_info(&mut self) -> Result<man::NetworkKeyInfo, Error> {
        self.communicate::<_, get_network_key_info::Response>(get_network_key_info::Command)
            .await?
            .try_into()
    }

    async fn import_key(
        &mut self,
        context: man::Context<Eui64>,
        key: man::Key,
    ) -> Result<(), Error> {
        self.communicate::<_, import_key::Response>(import_key::Command::new(context, key))
            .await?
            .try_into()
    }

    async fn import_link_key(
        &mut self,
        index: u8,
        address: Eui64,
        plaintext_key: man::Key,
    ) -> Result<(), Error> {
        self.communicate::<_, import_link_key::Response>(import_link_key::Command::new(
            index,
            address,
            plaintext_key,
        ))
        .await?
        .try_into()
    }

    async fn import_transient_key(
        &mut self,
        context: man::Context<Eui64>,
        eui64: Eui64,
        plaintext_key: man::Key,
        flags: man::Flags,
    ) -> Result<(), Error> {
        self.communicate::<_, import_transient_key::Response>(import_transient_key::Command::new(
            context,
            eui64,
            plaintext_key,
            flags,
        ))
        .await?
        .try_into()
    }

    async fn request_link_key(&mut self, partner: Eui64) -> Result<(), Error> {
        self.communicate::<_, request_link_key::Response>(request_link_key::Command::new(partner))
            .await?
            .try_into()
    }

    async fn send_trust_center_link_key(
        &mut self,
        destination_node_id: NodeId,
        destination_eui64: Eui64,
    ) -> Result<(), Error> {
        self.communicate::<_, send_trust_center_link_key::Response>(
            send_trust_center_link_key::Command::new(destination_node_id, destination_eui64),
        )
        .await?
        .try_into()
    }

    async fn set_initial_security_state(
        &mut self,
        state: security::initial::State,
    ) -> Result<(), Error> {
        self.communicate::<_, set_initial_security_state::Response>(
            set_initial_security_state::Command::new(state),
        )
        .await?
        .try_into()
    }

    async fn update_tc_link_key(&mut self, max_attempts: u8) -> Result<(), Error> {
        self.communicate::<_, update_tc_link_key::Response>(update_tc_link_key::Command::new(
            max_attempts,
        ))
        .await?
        .try_into()
    }
}
