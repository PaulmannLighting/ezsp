use std::collections::BTreeMap;
use std::sync::Arc;

use log::{debug, info};
use macaddr::MacAddr8;
use rand::random;
use silizium::zigbee::security::man::Key;
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, channel};
use zdp::SimpleDescriptor;
use zigbee_hw::{AwaitEvent, Event, EventTranslator, NcpDriver, NcpHandle, Start, bridge};

use super::event_handler::EventHandler;
use crate::ember::security::initial;
use crate::ember::{aps, concentrator, join, network};
use crate::ezsp::network::InitBitmask;
use crate::ezsp::{config, policy};
use crate::zigbee::EzspNetworkManager;
use crate::{
    Callback, Configuration, ConfigurationExt, Displayable, Error, Messaging, Networking,
    PolicyExt, Security, Transport, Utilities,
};

mod ashv2;

const RADIO_CHANNEL: u8 = 11;
const RADIO_POWER: i8 = 8;

/// Builder for Zigbee device configuration.
pub struct Builder<T> {
    transport: T,
    callbacks: Receiver<Callback>,
    policy: BTreeMap<policy::Id, u8>,
    configuration: BTreeMap<config::Id, u16>,
    concentrator: Option<concentrator::Parameters>,
    init_bitmask: InitBitmask,
    aps_options: aps::Options,
    link_key: Option<Key>,
    network_key: Option<Key>,
    join_method: join::Method,
    pan_id: Option<u16>,
    ieee_address: Option<MacAddr8>,
    radio_channel: u8,
    radio_power: i8,
    reinitialize: bool,
    buffers: usize,
}

impl<T> Builder<T> {
    /// Creates a new `Builder` with the given transport.
    #[must_use]
    pub const fn new(transport: T, callbacks: Receiver<Callback>) -> Self {
        Self {
            transport,
            callbacks,
            policy: BTreeMap::new(),
            configuration: BTreeMap::new(),
            concentrator: None,
            init_bitmask: InitBitmask::NO_OPTIONS,
            aps_options: aps::Options::empty(),
            link_key: None,
            network_key: None,
            join_method: join::Method::MacAssociation,
            pan_id: None,
            ieee_address: None,
            radio_channel: RADIO_CHANNEL,
            radio_power: RADIO_POWER,
            reinitialize: false,
            buffers: 1024,
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

    /// Sets the APS options.
    #[must_use]
    pub const fn with_aps_options(mut self, options: aps::Options) -> Self {
        self.aps_options = options;
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

    /// Sets whether to reinitialize the network.
    #[must_use]
    pub const fn with_reinitialize(mut self, reinitialize: bool) -> Self {
        self.reinitialize = reinitialize;
        self
    }

    /// Set the channels' buffer size.
    #[must_use]
    pub const fn with_buffers(mut self, buffers: usize) -> Self {
        self.buffers = buffers;
        self
    }
}

impl<T> Start for Builder<T>
where
    T: Transport + Sync + 'static,
{
    /// Starts the network manager on the given transport implementation.
    #[expect(clippy::too_many_lines)]
    async fn start(
        mut self,
        endpoints: &[SimpleDescriptor],
    ) -> Result<(NcpHandle, Receiver<Event>), zigbee_hw::Error> {
        if endpoints.is_empty() {
            return Err(zigbee_hw::Error::NoEndpoints);
        }

        let (message_tx, message_rx) = channel(self.buffers);
        spawn(bridge(self.callbacks, message_tx.clone()));
        let (events_tx, mut events_rx) = channel(self.buffers);
        let event_mux_handle = spawn(EventHandler::new(events_tx).run(message_rx));

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

        for (index, endpoint) in endpoints.iter().enumerate() {
            debug!(
                "Adding endpoint: {index:#04X}, profile: {:?}, device_id: {:#04X}, app_flags: {:#04X}, input clusters: {:X?}, output clusters: {:X?}",
                endpoint.profile_id(),
                endpoint.device_id(),
                endpoint.app_flags(),
                endpoint.input_clusters(),
                endpoint.output_clusters(),
            );
            self.transport
                .add_endpoint(
                    u8::try_from(index).map_err(implementation_error)?,
                    endpoint.profile_id(),
                    endpoint.device_id(),
                    endpoint.app_flags(),
                    endpoint.input_clusters().iter().copied().collect(),
                    endpoint.output_clusters().iter().copied().collect(),
                )
                .await?;
        }

        let ieee_address = self.transport.get_eui64().await?;
        debug!("IEEE address: {ieee_address}");

        let network_state = self.transport.network_state().await?;
        info!("Current network state: {network_state:?}");

        if self.reinitialize {
            if self.transport.leave_network().await.is_ok() {
                events_rx
                    .network_down()
                    .await
                    .expect("Events channel must be open at this point.");
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

        events_rx
            .network_up()
            .await
            .expect("Events channel must be open at this point.");
        info!("Network is up.");

        debug!("Setting radio power to {}", self.radio_power);
        self.transport.set_radio_power(self.radio_power).await?;

        let network_state = self.transport.network_state().await?;
        info!("Final network state: {network_state:?}");

        let (typ, parameters) = self.transport.get_network_parameters().await?;
        info!("Device type: {typ}");
        info!("Network parameters:\n{parameters}");

        log_state(&mut self.transport).await?;

        let radius = self
            .transport
            .get_configuration_value(config::Id::MaxHops)
            .await?;
        info!("Sending many-to-one route request: {radius} hops");
        #[expect(clippy::cast_possible_truncation)]
        self.transport
            .send_many_to_one_route_request(concentrator::Type::HighRam, radius as u8)
            .await?;

        let (_handle, ncp) = EzspNetworkManager::new(
            self.transport,
            // FIXME: Use first valid endpoint for now.
            endpoints
                .iter()
                .find_map(|endpoint| endpoint.profile().ok())
                .ok_or(zigbee_hw::Error::NoEndpoints)?,
            self.aps_options,
            message_tx,
            event_mux_handle,
        )
        .spawn(self.buffers);
        Ok((ncp, events_rx))
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

async fn log_state<T>(transport: &mut T) -> Result<(), Error>
where
    T: Transport,
{
    debug!(
        "Configuration:\n{}",
        transport.get_configuration().await?.displayable()
    );

    debug!(
        "Policies:\n{}",
        transport.get_policies().await?.displayable()
    );

    info!(
        "Current security state:\n{}",
        transport.get_current_security_state().await?
    );

    Ok(())
}

fn implementation_error<T>(error: T) -> zigbee_hw::Error
where
    T: std::error::Error + Send + Sync + 'static,
{
    zigbee_hw::Error::Implementation(Arc::new(error))
}
