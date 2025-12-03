//! Network manager for Zigbee networks.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::time::Duration;

use enum_iterator::all;
use le_stream::ToLeStream;
use log::{debug, info, warn};
use macaddr::MacAddr8;
use tokio::time::sleep;
use zigbee_nwk::aps::Command;
use zigbee_nwk::zcl::Cluster;
use zigbee_nwk::{Nlme, zcl};

pub use self::device_config::DeviceConfig;
pub use self::event_handler::EventHandler;
pub use self::zigbee_message::ZigbeeMessage;
use crate::ember::message::Destination;
use crate::ember::security::initial;
use crate::ember::{aps, concentrator, join, network};
use crate::error::Status;
use crate::ezsp::network::InitBitmask;
use crate::ezsp::{config, decision, policy};
use crate::{Configuration, Error, Messaging, Networking, Security, Utilities, ember};

mod address;
mod device_config;
mod event_handler;
mod zigbee_message;

const ENDPOINT_ID: u8 = 1;
const HOME_AUTOMATION: u16 = 0x0104;
const HOME_GATEWAY: u16 = 0x0050;
const INPUT_CLUSTERS: &[u16] = &[0x0000, 0x0006, 0x0008, 0x0300, 0x0403, 0x0201];
const OUTPUT_CLUSTERS: &[u16] = &[0x0000, 0x0006, 0x0008, 0x0300, 0x0403];

/// Network manager for Zigbee networks.
pub struct NetworkManager<T> {
    transport: T,
    settings: Option<DeviceConfig>,
    network_up: Arc<AtomicBool>,
    network_open: Arc<AtomicBool>,
    message_tag: u8,
    aps_options: aps::Options,
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
            settings: None,
            network_up,
            network_open,
            message_tag: 0,
            aps_options: aps::Options::empty(),
        }
    }

    /// Returns the next message tag and increments the internal counter.
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
    /// Configures the network manager with the given device settings.
    pub async fn configure(&mut self, settings: DeviceConfig) -> Result<(), Error> {
        info!("Initial EZSP NCP configuration:");
        for (key, value) in self.get_configuration().await {
            info!("  {key:?}: {value}");
        }

        info!("Initial EZSP NCP policy:");
        for (key, value) in self.get_policy().await {
            info!("  {key:?}: {value:?}");
        }

        self.settings.replace(settings);

        info!("Initialization complete");
        Ok(())
    }

    /// Starts the network manager, optionally reinitializing the network.
    pub async fn start(&mut self, reinitialize: bool) -> Result<(), Error> {
        let Some(settings) = self.settings.clone() else {
            return Err(Error::NotConfigured);
        };

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

        if !reinitialize {
            info!("Initializing network manager");
            self.initialize(
                settings.concentrator.clone(),
                settings.configuration.clone(),
                settings.policy.clone(),
                settings.link_key,
                settings.network_key,
            )
            .await?;
        }

        info!("Initializing network");
        if let Err(error) = self.transport.network_init(InitBitmask::empty()).await {
            match error {
                Error::Status(Status::Ember(Ok(ember::Status::NotJoined))) => {
                    if !reinitialize {
                        return Err(error);
                    }
                }
                error => {
                    return Err(error);
                }
            }
        }

        let network_state = self.transport.network_state().await?;
        info!("Current network state: {network_state:?}");

        if reinitialize {
            self.leave().await?;

            info!("Initializing network manager");
            self.initialize(
                settings.concentrator,
                settings.configuration,
                settings.policy,
                settings.link_key,
                settings.network_key,
            )
            .await?;

            info!("Reinitializing network");
            self.transport
                .form_network(network::Parameters::new(
                    settings.extended_pan_id,
                    settings.pan_id,
                    settings.radio_power as u8,
                    settings.radio_channel,
                    join::Method::MacAssociation,
                    0,
                    0,
                    1 << settings.radio_channel,
                ))
                .await?;
        }

        self.await_network_up().await;

        let network_state = self.transport.network_state().await?;
        info!("Final network state: {network_state:?}");

        let security_state = self.transport.get_current_security_state().await?;
        info!("Current security state: {security_state:?}");
        for (name, _) in security_state.bitmask().iter_names() {
            info!("  Security bitmask: {name}");
        }

        info!("Sending many-to-one route request");
        let radius = self
            .transport
            .get_configuration_value(config::Id::MaxHops)
            .await?;
        self.transport
            .send_many_to_one_route_request(concentrator::Type::HighRam, radius as u8)
            .await?;

        Ok(())
    }

    async fn initialize(
        &mut self,
        concentrator: concentrator::Parameters,
        configuration: BTreeMap<config::Id, u16>,
        policy: BTreeMap<policy::Id, u8>,
        link_key: [u8; 16],
        network_key: [u8; 16],
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
                initial::Bitmask::TRUST_CENTER_GLOBAL_LINK_KEY
                    | initial::Bitmask::HAVE_NETWORK_KEY
                    | initial::Bitmask::HAVE_PRECONFIGURED_KEY
                    | initial::Bitmask::REQUIRE_ENCRYPTED_KEY,
                link_key,
                network_key,
                0,
                MacAddr8::default(),
            ))
            .await?;

        Ok(())
    }

    async fn leave(&mut self) -> Result<(), Error> {
        if self.transport.leave_network().await.is_ok() {
            info!("Waiting for network to go down...");
            while self.network_up.load(SeqCst) {
                sleep(Duration::from_secs(1)).await;
            }
        }

        Ok(())
    }

    async fn await_network_up(&mut self) {
        info!("Waiting for network to come up...");
        while !self.network_up.load(SeqCst) {
            sleep(Duration::from_secs(1)).await;
        }
        info!("Network is up");

        info!("Final EZSP NCP configuration:");
        for (key, value) in self.get_configuration().await {
            info!("  {key:?}: {value}");
        }

        info!("Final EZSP NCP policy:");
        for (key, value) in self.get_policy().await {
            info!("  {key:?}: {value:?}");
        }

        let (node_type, parameters) = self
            .transport
            .get_network_parameters()
            .await
            .expect("Failed to get network parameters");
        info!("Node type: {node_type:?}");
        info!("{parameters}");
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
    async fn set_stack_policy(&mut self, policy: BTreeMap<policy::Id, u8>) -> Result<(), Error> {
        for (key, value) in policy {
            info!("Setting policy {key:?} to {value:?}");
            self.transport.set_policy(key, value).await?;
        }

        Ok(())
    }

    async fn get_configuration(&mut self) -> BTreeMap<config::Id, u16> {
        let mut configuration = BTreeMap::new();

        for id in all::<config::Id>() {
            match self.transport.get_configuration_value(id).await {
                Ok(value) => {
                    configuration.insert(id, value);
                }
                Err(error) => {
                    warn!("Failed to get configuration {id:?}: {error}");
                }
            }
        }

        configuration
    }

    async fn get_policy(&mut self) -> BTreeMap<policy::Id, decision::Id> {
        let mut policy = BTreeMap::new();

        for id in all::<policy::Id>() {
            match self.transport.get_policy(id).await {
                Ok(value) => {
                    policy.insert(id, value);
                }
                Err(error) => {
                    warn!("Failed to get policy {id:?}: {error}");
                }
            }
        }

        policy
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
    T: Networking,
{
    pub async fn get_neighbors(&mut self) -> Result<BTreeMap<MacAddr8, u16>, Error> {
        let mut neighbors = BTreeMap::new();

        for index in 0..=u8::MAX {
            if let Ok(neighbor) = self.transport.get_neighbor(index).await {
                neighbors.insert(neighbor.long_id(), neighbor.short_id());
            } else {
                break;
            }
        }

        Ok(neighbors)
    }

    pub async fn get_children(&mut self) -> Result<BTreeMap<MacAddr8, u16>, Error> {
        let mut neighbors = BTreeMap::new();

        for index in 0..=u8::MAX {
            if let Ok(child) = self.transport.get_child_data(index).await {
                neighbors.insert(child.eui64(), child.id());
            } else {
                break;
            }
        }

        Ok(neighbors)
    }
}

impl<T> NetworkManager<T>
where
    T: Sync,
{
    /// Wait until the network is open.
    pub async fn await_network_open(&self) {
        while !self.network_open.load(SeqCst) {
            sleep(Duration::from_secs(1)).await;
        }
    }

    /// Wait until the network is closed.
    pub async fn await_network_closed(&self) {
        while self.network_open.load(SeqCst) {
            sleep(Duration::from_secs(1)).await;
        }
    }
}

impl<T> Nlme for NetworkManager<T>
where
    T: Configuration + Security + Messaging + Networking + Utilities,
{
    type Error = Error;

    async fn unicast_command<P>(
        &mut self,
        destination: u16,
        frame: Command<P>,
    ) -> Result<(), zigbee_nwk::Error<Self::Error>>
    where
        P: zcl::Command + ToLeStream,
    {
        let tag = self.next_message_tag();
        let seq = self
            .transport
            .send_unicast(
                Destination::Direct(destination),
                aps::Frame::new(
                    HOME_AUTOMATION,
                    <P as Cluster>::ID,
                    0x01,
                    0x01,
                    self.aps_options,
                    0x00,
                    0x00,
                ),
                tag,
                frame.into_payload().to_le_stream().collect(),
            )
            .await?;
        debug!("Received seq: {seq}");
        Ok(())
    }
}
