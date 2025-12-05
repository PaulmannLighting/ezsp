use std::collections::{BTreeMap, BTreeSet};
use std::iter::once;

use log::{debug, info};
use macaddr::MacAddr8;
use rand::random;
use silizium::zigbee::security::man::Key;
use tokio_mpmc::Receiver;

use crate::ember::security::initial;
use crate::ember::{aps, concentrator, join, network};
use crate::ezsp::network::InitBitmask;
use crate::ezsp::{config, policy};
use crate::zigbee::NetworkManager;
use crate::zigbee::network_manager::stack_status::StackStatus;
use crate::{
    Callback, Configuration, Error, Messaging, Networking, Security, Transport, Utilities, ember,
};

const HOME_AUTOMATION: u16 = 0x0104;
const HOME_GATEWAY: u16 = 0x0050;
const INPUT_CLUSTERS: &[u16] = &[0x0000, 0x0006, 0x0008, 0x0300, 0x0403, 0x0201];
const OUTPUT_CLUSTERS: &[u16] = &[0x0000, 0x0006, 0x0008, 0x0300, 0x0403];
const RADIO_CHANNEL: u8 = 11;
const RADIO_POWER: i8 = 8;
const ENDPOINT_ID: u8 = 1;

/// Builder for Zigbee device configuration.
pub struct Builder<T> {
    transport: T,
    callbacks_rx: Receiver<Callback>,
    policy: BTreeMap<policy::Id, u8>,
    configuration: BTreeMap<config::Id, u16>,
    concentrator: Option<concentrator::Parameters>,
    init_bitmask: InitBitmask,
    app_flags: u8,
    aps_options: aps::Options,
    profile_id: u16,
    device_id: u16,
    input_clusters: Vec<u16>,
    output_clusters: Vec<u16>,
    link_key: Option<Key>,
    network_key: Option<Key>,
    join_method: join::Method,
    pan_id: Option<u16>,
    ieee_address: Option<MacAddr8>,
    radio_channel: u8,
    radio_power: i8,
    endpoints: BTreeSet<u8>,
    reinitialize: bool,
}

impl<T> Builder<T> {
    /// Creates a new `Builder` with the given transport.
    #[must_use]
    pub fn new(transport: T, callbacks_rx: Receiver<Callback>) -> Self {
        Self {
            transport,
            callbacks_rx,
            policy: BTreeMap::new(),
            configuration: BTreeMap::new(),
            concentrator: None,
            init_bitmask: InitBitmask::NO_OPTIONS,
            app_flags: 0,
            aps_options: aps::Options::empty(),
            profile_id: HOME_AUTOMATION,
            device_id: HOME_GATEWAY,
            input_clusters: INPUT_CLUSTERS.to_vec(),
            output_clusters: OUTPUT_CLUSTERS.to_vec(),
            link_key: None,
            network_key: None,
            join_method: join::Method::MacAssociation,
            pan_id: None,
            ieee_address: None,
            radio_channel: RADIO_CHANNEL,
            radio_power: RADIO_POWER,
            endpoints: once(ENDPOINT_ID).collect(),
            reinitialize: false,
        }
    }

    /// Adds a policy decision to the configuration.
    #[must_use]
    pub fn with_policy(mut self, policy: policy::Id, decision: impl Into<u8>) -> Self {
        self.policy.insert(policy, decision.into());
        self
    }

    /// Adds multiple policy decisions to the configuration.
    #[must_use]
    pub fn with_policies(mut self, policies: BTreeMap<policy::Id, u8>) -> Self {
        self.policy.extend(policies);
        self
    }

    /// Adds a configuration value to the configuration.
    #[must_use]
    pub fn with_configuration(mut self, config: config::Id, value: u16) -> Self {
        self.configuration.insert(config, value);
        self
    }

    /// Adds multiple configuration values to the configuration.
    #[must_use]
    pub fn with_configurations(mut self, configurations: BTreeMap<config::Id, u16>) -> Self {
        self.configuration.extend(configurations);
        self
    }

    /// Sets the concentrator parameters for the configuration.
    #[must_use]
    pub const fn with_concentrator(mut self, concentrator: concentrator::Parameters) -> Self {
        self.concentrator.replace(concentrator);
        self
    }

    /// Sets the application flags for the configuration.
    #[must_use]
    pub const fn with_app_flags(mut self, flags: u8) -> Self {
        self.app_flags = flags;
        self
    }

    /// Sets the APS options.
    #[must_use]
    pub const fn with_aps_options(mut self, options: aps::Options) -> Self {
        self.aps_options = options;
        self
    }

    /// Sets the profile ID for the configuration.
    #[must_use]
    pub const fn with_profile_id(mut self, profile_id: u16) -> Self {
        self.profile_id = profile_id;
        self
    }

    /// Sets the device ID for the configuration.
    #[must_use]
    pub const fn with_device_id(mut self, device_id: u16) -> Self {
        self.device_id = device_id;
        self
    }

    /// Adds an input cluster to the configuration.
    #[must_use]
    pub fn with_input_cluster(mut self, input_cluster: u16) -> Self {
        self.input_clusters.push(input_cluster);
        self
    }

    /// Adds multiple input clusters to the configuration.
    #[must_use]
    pub fn with_input_clusters(mut self, input_clusters: &[u16]) -> Self {
        self.input_clusters.extend_from_slice(input_clusters);
        self
    }

    /// Adds an output cluster to the configuration.
    #[must_use]
    pub fn with_output_cluster(mut self, output_cluster: u16) -> Self {
        self.output_clusters.push(output_cluster);
        self
    }

    /// Adds multiple output clusters to the configuration.
    #[must_use]
    pub fn with_output_clusters(mut self, output_clusters: &[u16]) -> Self {
        self.output_clusters.extend_from_slice(output_clusters);
        self
    }

    /// Sets the link key for the configuration.
    #[must_use]
    pub const fn with_link_key(mut self, link_key: Key) -> Self {
        self.link_key.replace(link_key);
        self
    }

    /// Sets the network key for the configuration.
    #[must_use]
    pub const fn with_network_key(mut self, network_key: Key) -> Self {
        self.network_key.replace(network_key);
        self
    }

    /// Sets the join method for the configuration.
    #[must_use]
    pub const fn with_join_method(mut self, join_method: join::Method) -> Self {
        self.join_method = join_method;
        self
    }

    /// Sets the PAN ID for the configuration.
    #[must_use]
    pub const fn with_pan_id(mut self, pan_id: u16) -> Self {
        self.pan_id.replace(pan_id);
        self
    }

    /// Sets the IEEE address for the configuration.
    #[must_use]
    pub const fn with_ieee_address(mut self, ieee_address: MacAddr8) -> Self {
        self.ieee_address.replace(ieee_address);
        self
    }

    /// Sets the radio channel for the configuration.
    #[must_use]
    pub const fn with_radio_channel(mut self, radio_channel: u8) -> Self {
        self.radio_channel = radio_channel;
        self
    }

    /// Sets the radio power for the configuration.
    #[must_use]
    pub const fn with_radio_power(mut self, radio_power: i8) -> Self {
        self.radio_power = radio_power;
        self
    }

    /// Sets the endpoint ID for the configuration.
    #[must_use]
    pub fn with_endpoint(mut self, endpoint_id: u8) -> Self {
        self.endpoints.insert(endpoint_id);
        self
    }

    /// Sets multiple endpoint IDs for the configuration.
    #[must_use]
    pub fn with_endpoints(mut self, endpoint_ids: &[u8]) -> Self {
        self.endpoints.extend(endpoint_ids.iter().copied());
        self
    }

    /// Sets whether to reinitialize the network.
    #[must_use]
    pub const fn with_reinitialize(mut self, reinitialize: bool) -> Self {
        self.reinitialize = reinitialize;
        self
    }

    /// Starts the network manager on the given transport implementation.
    pub async fn start(mut self) -> Result<(NetworkManager<T>, Receiver<Callback>), Error>
    where
        T: Transport,
    {
        debug!("Setting concentrator");
        self.transport.set_concentrator(self.concentrator).await?;

        for (key, value) in self.configuration {
            debug!("Setting configuration {key:?} to {value:#06X}");
            self.transport.set_configuration_value(key, value).await?;
        }

        for (key, value) in self.policy {
            debug!("Setting policy {key:?} to {value:#04X}");
            self.transport.set_policy(key, value).await?;
        }

        for endpoint in self.endpoints {
            debug!("Adding endpoint: {endpoint:#04X}");
            self.transport
                .add_endpoint(
                    endpoint,
                    self.profile_id,
                    self.device_id,
                    0,
                    self.input_clusters.iter().copied().collect(),
                    self.output_clusters.iter().copied().collect(),
                )
                .await?;
        }

        let ieee_address = self.transport.get_eui64().await?;
        debug!("IEEE address: {ieee_address}");

        let network_state = self.transport.network_state().await?;
        info!("Current network state: {network_state:?}");

        if self.reinitialize {
            if self.transport.leave_network().await.is_ok() {
                self.callbacks_rx
                    .stack_status(ember::Status::NotJoined)
                    .await?;
                info!("Left existing network.");
            }

            debug!("Setting initial security state");
            self.transport
                .set_initial_security_state(build_initial_security_state(
                    self.link_key,
                    self.network_key,
                ))
                .await?;

            info!("Reinitializing network");
            #[expect(clippy::cast_sign_loss)]
            self.transport
                .form_network(network::Parameters::new(
                    self.ieee_address.unwrap_or_default(),
                    self.pan_id.unwrap_or_else(random),
                    self.radio_power as u8,
                    self.radio_channel,
                    self.join_method,
                    0,
                    0,
                    1 << self.radio_channel,
                ))
                .await?;
        } else {
            self.transport.network_init(self.init_bitmask).await?;
        }

        self.callbacks_rx
            .stack_status(ember::Status::NetworkUp)
            .await?;
        info!("Network is up.");

        debug!("Setting radio power to {}", self.radio_power);
        self.transport.set_radio_power(self.radio_power).await?;

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
        #[expect(clippy::cast_possible_truncation)]
        self.transport
            .send_many_to_one_route_request(concentrator::Type::HighRam, radius as u8)
            .await?;

        let network_manager =
            NetworkManager::new(self.transport, self.profile_id, self.aps_options);

        Ok((network_manager, self.callbacks_rx))
    }
}

fn build_initial_security_state(link_key: Option<Key>, network_key: Option<Key>) -> initial::State {
    let mut initial_security_state_bitmask = initial::Bitmask::TRUST_CENTER_GLOBAL_LINK_KEY;

    let link_key = link_key.map_or_else(Key::default, |link_key| {
        initial_security_state_bitmask |=
            initial::Bitmask::HAVE_PRECONFIGURED_KEY | initial::Bitmask::REQUIRE_ENCRYPTED_KEY;
        link_key
    });

    let network_key = network_key.map_or_else(Key::default, |network_key| {
        initial_security_state_bitmask |= initial::Bitmask::HAVE_NETWORK_KEY;
        network_key
    });

    initial::State::new(
        initial_security_state_bitmask,
        link_key,
        network_key,
        0,
        MacAddr8::default(),
    )
}
