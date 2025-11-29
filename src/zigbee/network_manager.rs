//! Network manager for Zigbee networks.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::time::Duration;

use enum_iterator::all;
pub use event_handler::EventHandler;
use log::{info, warn};
use macaddr::MacAddr8;
pub use send_unicast::SendUnicast;
use tokio::time::sleep;
pub use zigbee_message::ZigbeeMessage;

use crate::ember::message::Destination;
use crate::ember::security::initial;
use crate::ember::{aps, concentrator, join, network};
use crate::ezsp::{config, decision, policy};
use crate::{Configuration, Error, Messaging, Networking, Security, Utilities};

mod address;
mod endpoint;
mod event_handler;
mod send_unicast;
mod zigbee_message;

const ENDPOINT_ID: u8 = 1;
const RADIO_TX_POWER: i8 = 8;
const NETWORK_KEY: [u8; 16] = [
    0x29, 0xB0, 0x0D, 0xE6, 0x31, 0xAB, 0x7A, 0xD0, 0xC6, 0x83, 0xC8, 0x7A, 0xBF, 0x70, 0xD6, 0x08,
];
const HOME_AUTOMATION: u16 = 0x0104;
const HOME_GATEWAY: u16 = 0x0050;
const INPUT_CLUSTERS: &[u16] = &[0x0000, 0x0006, 0x0008, 0x0300, 0x0403, 0x0201];
const OUTPUT_CLUSTERS: &[u16] = &[0x0000, 0x0006, 0x0008, 0x0300, 0x0403];
const TICK: Duration = Duration::from_secs(1);

/// Network manager for Zigbee networks.
pub struct NetworkManager<T> {
    transport: T,
    network_up: Arc<AtomicBool>,
    network_open: Arc<AtomicBool>,
    aps_seq: u8,
    message_tag: u8,
}

impl<T> NetworkManager<T> {
    /// Creates a new `NetworkManager` with the given transport.
    #[must_use]
    pub const fn new(
        transport: T,
        network_up: Arc<AtomicBool>,
        network_open: Arc<AtomicBool>,
    ) -> Self {
        Self {
            transport,
            network_up,
            network_open,
            aps_seq: 0,
            message_tag: 0,
        }
    }

    const fn next_aps_seq(&mut self) -> u8 {
        let seq = self.aps_seq;
        self.aps_seq = self.aps_seq.wrapping_add(1);
        seq
    }

    const fn next_message_tag(&mut self) -> u8 {
        let tag = self.message_tag;
        self.message_tag = self.message_tag.wrapping_add(1);
        tag
    }
}

impl<T> NetworkManager<T>
where
    T: Configuration + Security + Messaging + Networking + Utilities,
{
    /// Initializes the Zigbee endpoint.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the endpoint could not be added.
    #[expect(clippy::too_many_arguments)]
    pub async fn init(
        &mut self,
        reinitialize: bool,
        concentrator: concentrator::Parameters,
        configuration: BTreeMap<config::Id, u16>,
        policy: BTreeMap<policy::Id, decision::Id>,
        link_key: [u8; 16],
        extended_pan_id: MacAddr8,
        pan_id: u16,
        radio_channel: u8,
    ) -> Result<(), Error> {
        info!("Initial EZSP NCP configuration:");
        self.print_configuration().await;
        info!("Initial EZSP NCP policy:");
        self.print_policy().await;

        info!("Adding endpoint");
        self.transport
            .add_endpoint(
                ENDPOINT_ID,
                HOME_AUTOMATION,
                HOME_GATEWAY,
                0,
                INPUT_CLUSTERS.iter().copied().collect(),
                OUTPUT_CLUSTERS.iter().copied().collect(),
            )
            .await?;

        if reinitialize {
            info!("Leaving existing network");

            if matches!(self.transport.leave_network().await, Ok(())) {
                while self.network_up.load(SeqCst) {
                    sleep(TICK).await;
                }

                info!("Left existing network");
            }
        }

        info!("Initializing network");
        self.initialize(concentrator, configuration, policy, link_key)
            .await?;

        if reinitialize {
            info!("Reinitializing network");
            self.transport
                .form_network(network::Parameters::new(
                    extended_pan_id,
                    pan_id,
                    RADIO_TX_POWER as u8,
                    radio_channel,
                    join::Method::MacAssociation,
                    0,
                    0,
                    0,
                ))
                .await?;
        } else {
            self.transport.network_init(BTreeSet::default()).await?;
        }

        self.await_network_up().await;

        info!("Sending many-to-one route request");
        self.transport
            .send_many_to_one_route_request(concentrator::Type::HighRam, 8)
            .await?;

        Ok(())
    }

    async fn initialize(
        &mut self,
        concentrator: concentrator::Parameters,
        configuration: BTreeMap<config::Id, u16>,
        policy: BTreeMap<policy::Id, decision::Id>,
        link_key: [u8; 16],
    ) -> Result<(), Error> {
        info!("Initializing EZSP NCP");
        self.transport.set_concentrator(Some(concentrator)).await?;
        self.set_stack_configuration(configuration).await?;
        self.set_stack_policy(policy).await?;

        let ieee_address = self.transport.get_eui64().await?;
        info!("IEEE address: {ieee_address}");

        self.transport.set_radio_power(8).await?;

        self.transport
            .set_initial_security_state(initial::State::new(
                [
                    initial::Bitmask::HavePreconfiguredKey,
                    initial::Bitmask::RequireEncryptedKey,
                ]
                .into(),
                link_key,
                NETWORK_KEY,
                0,
                MacAddr8::default(),
            ))
            .await?;

        Ok(())
    }

    async fn await_network_up(&mut self) {
        info!("Waiting for network to come up...");
        while !self.network_up.load(SeqCst) {
            sleep(Duration::from_secs(1)).await;
        }
        info!("Network is up");

        info!("Final EZSP NCP configuration:");
        self.print_configuration().await;
        info!("Final EZSP NCP policy:");
        self.print_policy().await;

        let (node_type, parameters) = self
            .transport
            .get_network_parameters()
            .await
            .expect("Failed to get network parameters");
        info!("Node type: {node_type:?}");
        log_parameters(&parameters);
    }
}

impl<T> NetworkManager<T>
where
    T: Configuration,
{
    /// Sets the stack configuration according to the given map of configuration IDs to values.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any of the configuration values could not be set.
    async fn set_stack_configuration(
        &mut self,
        configuration: BTreeMap<config::Id, u16>,
    ) -> Result<(), Error> {
        for (key, value) in configuration {
            info!("Setting configuration {key:?} to {value}");
            self.transport.set_configuration_value(key, value).await?;
        }

        Ok(())
    }

    /// Sets the stack policy according to the given map of policy IDs to decision IDs.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any of the policies could not be set.
    async fn set_stack_policy(
        &mut self,
        policy: BTreeMap<policy::Id, decision::Id>,
    ) -> Result<(), Error> {
        for (key, value) in policy {
            info!("Setting policy {key:?} to {value:?}");
            self.transport.set_policy(key, value).await?;
        }

        Ok(())
    }

    async fn print_configuration(&mut self) {
        for id in all::<config::Id>() {
            match self.transport.get_configuration_value(id).await {
                Ok(value) => info!("Configuration {id:?} = {value}"),
                Err(error) => warn!("Failed to get configuration {id:?}: {error}"),
            }
        }
    }

    async fn print_policy(&mut self) {
        for id in all::<policy::Id>() {
            match self.transport.get_policy(id).await {
                Ok(value) => info!("Policy {id:?} = {value:?}"),
                Err(error) => warn!("Failed to get policy {id:?}: {error}"),
            }
        }
    }
}

impl<T> NetworkManager<T>
where
    T: Networking,
{
    /// Allows joining for the given duration.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if joining could not be permitted.
    pub async fn allow_joins(&mut self, duration: network::Duration) -> Result<(), Error> {
        info!("Allowing joins for {} seconds.", u8::from(duration));
        self.transport.permit_joining(duration).await
    }
}

impl<T> NetworkManager<T>
where
    T: Messaging,
{
    /// Sends a unicast message to the given destination with the given APS frame and payload.
    ///
    /// Returns the message tag used for the message.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the message could not be sent.
    pub async fn send_unicast(
        &mut self,
        destination: Destination,
        aps_frame: aps::Frame,
        payload: &[u8],
    ) -> Result<u8, Error> {
        let tag = self.next_message_tag();
        self.transport
            .send_unicast(
                destination,
                aps_frame,
                tag,
                payload.iter().copied().collect(),
            )
            .await
    }
}

impl<T> NetworkManager<T>
where
    T: Sync,
{
    /// Wait until the network is open.
    pub async fn await_network_open(&self) {
        info!("Waiting for network to open...");
        while !self.network_open.load(SeqCst) {
            sleep(Duration::from_secs(1)).await;
        }
        info!("Network is open");
    }

    /// Wait until the network is closed.
    pub async fn await_network_closed(&self) {
        info!("Waiting for network to close...");
        while self.network_open.load(SeqCst) {
            sleep(Duration::from_secs(1)).await;
        }
        info!("Network is closed");
    }
}

fn log_parameters(parameters: &network::Parameters) {
    info!("PAN ID: {:#X}", parameters.pan_id());
    info!("Extended PAN ID: {:#X?}", parameters.extended_pan_id());
    info!("Radio TX power: {:#X}", parameters.radio_tx_power());
    info!("Radio channel: {:#X}", parameters.radio_channel());
    info!("Join method: {:#X?}", parameters.join_method());
    info!("Nwk manager ID: {:#X}", parameters.nwk_manager_id());
    info!("Nwk update ID: {:#X}", parameters.nwk_update_id());
    info!("Channels: {:#X}", parameters.channels());
}
