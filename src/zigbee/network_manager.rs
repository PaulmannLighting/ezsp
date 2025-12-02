//! Network manager for Zigbee networks.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::time::Duration;

use enum_iterator::all;
use log::{info, warn};
use macaddr::MacAddr8;
use tokio::time::sleep;
use zigbee_nwk::{NetworkDescriptor, Nlme};

pub use self::device_config::DeviceConfig;
pub use self::event_handler::EventHandler;
pub use self::zigbee_message::ZigbeeMessage;
use crate::ember::message::Destination;
use crate::ember::security::initial;
use crate::ember::{aps, concentrator, join, network};
use crate::ezsp::{config, decision, policy};
use crate::{Configuration, Error, Messaging, Networking, Security, Utilities};

mod address;
mod device_config;
mod event_handler;
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
    settings: Option<DeviceConfig>,
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
            settings: None,
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

    pub async fn send_raw(&mut self, payload: &[u8]) -> Result<(), Error> {
        self.transport
            .send_raw_message(payload.iter().copied().collect())
            .await
    }
}

impl<T> NetworkManager<T>
where
    T: Messaging,
{
    pub async fn send_many_to_one_route_request(&mut self) -> Result<(), Error> {
        info!("Sending many-to-one route request");
        self.transport
            .send_many_to_one_route_request(concentrator::Type::HighRam, 8)
            .await
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
    type DeviceSettings = DeviceConfig;
    type Error = Error;

    async fn configure(&mut self, settings: Self::DeviceSettings) -> Result<(), Self::Error> {
        info!("Initial EZSP NCP configuration:");
        for (key, value) in self.get_configuration().await {
            info!("  {key:?}: {value}");
        }

        info!("Initial EZSP NCP policy:");
        for (key, value) in self.get_policy().await {
            info!("  {key:?}: {value:?}");
        }

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

        self.settings.replace(settings);
        Ok(())
    }

    async fn start(&mut self, reinitialize: bool) -> Result<(), Self::Error> {
        let Some(settings) = self.settings.clone() else {
            return Err(Error::NotConfigured);
        };

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
        self.initialize(
            settings.concentrator,
            settings.configuration,
            settings.policy,
            settings.link_key,
        )
        .await?;

        if reinitialize {
            info!("Reinitializing network");
            self.transport
                .form_network(network::Parameters::new(
                    settings.extended_pan_id,
                    settings.pan_id,
                    RADIO_TX_POWER as u8,
                    settings.radio_channel,
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
        self.send_many_to_one_route_request().await?;

        Ok(())
    }

    async fn join(&mut self, network: NetworkDescriptor) -> Result<(), Self::Error> {
        todo!("Network joining is not yet implemented")
    }

    async fn rejoin(&mut self, network: NetworkDescriptor) -> Result<(), Self::Error> {
        todo!("Network rejoining is not yet implemented")
    }

    async fn leave(&mut self) -> Result<(), Self::Error> {
        self.transport.leave_network().await
    }
}
