use std::collections::BTreeMap;
use std::num::NonZero;

use log::{debug, info, trace};
use tokio::spawn;
use tokio::sync::mpsc::{Sender, channel};

use crate::ember::{aps, concentrator};
use crate::ezsp::{config, policy};
use crate::ncp::Endpoint;
use crate::ncp::await_event::AwaitEvent;
use crate::{
    Configuration, ConfigurationExt, Connected, Displayable, Error, EventHandler,
    MIN_NON_LEGACY_VERSION, Messaging, Ncp, Networking, PolicyExt, Receive, Security, Startup,
    Transceiver, TranslatableEvent, Transmit, Utilities,
};

const RADIO_POWER: i8 = 8;

/// Configures and starts an actor-backed EZSP Network Co-Processor.
///
/// The builder owns a [`Transceiver`] until [`Builder::start`] spawns its
/// transmitter and receiver tasks. It also stores queue capacities, the desired
/// EZSP version, stack policies and configuration values, concentrator
/// settings, radio power, and default APS options.
pub struct Builder<T, R> {
    pub(crate) transceiver: Transceiver<T, R>,
    pub(crate) callbacks_capacity: usize,
    pub(crate) messages_capacity: usize,
    pub(crate) desired_version: NonZero<u8>,
    pub(crate) policy: BTreeMap<policy::Id, u8>,
    pub(crate) configuration: BTreeMap<config::Id, u16>,
    pub(crate) concentrator: Option<concentrator::Parameters>,
    pub(crate) radio_tx_power: i8,
    pub(crate) aps_options: aps::Options,
}

impl<T, R> Builder<T, R> {
    /// Creates a builder around separate transport transmit and receive halves.
    ///
    /// # Panics
    ///
    /// Panics if [`MIN_NON_LEGACY_VERSION`] is zero. The protocol constant is
    /// defined as a nonzero version, so this indicates an invalid crate build.
    #[must_use]
    pub const fn new(transceiver: Transceiver<T, R>) -> Self {
        Self {
            transceiver,
            callbacks_capacity: 128,
            messages_capacity: 64,
            desired_version: NonZero::new(MIN_NON_LEGACY_VERSION)
                .expect("Min legacy version is non-zero."),
            policy: BTreeMap::new(),
            configuration: BTreeMap::new(),
            concentrator: None,
            radio_tx_power: RADIO_POWER,
            aps_options: aps::Options::empty(),
        }
    }

    /// Sets the capacity of the asynchronous callback channel.
    #[must_use]
    pub const fn with_callbacks_capacity(mut self, callbacks_capacity: usize) -> Self {
        self.callbacks_capacity = callbacks_capacity;
        self
    }

    /// Sets the capacity of actor command and response message channels.
    #[must_use]
    pub const fn with_messages_capacity(mut self, messages_capacity: usize) -> Self {
        self.messages_capacity = messages_capacity;
        self
    }

    /// Sets the EZSP protocol version requested during connection negotiation.
    #[must_use]
    pub const fn with_desired_version(mut self, desired_version: NonZero<u8>) -> Self {
        self.desired_version = desired_version;
        self
    }

    /// Adds one EZSP policy decision to apply during startup.
    #[must_use]
    pub fn with_policy(mut self, policy: policy::Id, decision: impl Into<u8>) -> Self {
        self.policy.insert(policy, decision.into());
        self
    }

    /// Adds multiple EZSP policy decisions to apply during startup.
    #[must_use]
    pub fn with_policies(mut self, policies: BTreeMap<policy::Id, u8>) -> Self {
        self.policy.extend(policies);
        self
    }

    /// Adds one EZSP configuration value to apply during startup.
    #[must_use]
    pub fn with_configuration(mut self, config: config::Id, value: u16) -> Self {
        self.configuration.insert(config, value);
        self
    }

    /// Adds multiple EZSP configuration values to apply during startup.
    #[must_use]
    pub fn with_configurations(mut self, configurations: BTreeMap<config::Id, u16>) -> Self {
        self.configuration.extend(configurations);
        self
    }

    /// Sets the many-to-one route concentrator parameters.
    #[must_use]
    pub const fn with_concentrator(mut self, concentrator: concentrator::Parameters) -> Self {
        self.concentrator.replace(concentrator);
        self
    }

    /// Sets the radio transmit power used during network formation and after startup.
    #[must_use]
    pub const fn with_radio_tx_power(mut self, radio_tx_power: i8) -> Self {
        self.radio_tx_power = radio_tx_power;
        self
    }

    /// Sets the default APS options for outgoing APS messages created by [`Ncp`](Ncp).
    #[must_use]
    pub const fn with_aps_options(mut self, options: aps::Options) -> Self {
        self.aps_options = options;
        self
    }
}

impl<T, R> Builder<T, R>
where
    T: Transmit + Send + 'static,
    R: Receive + Send + 'static,
{
    /// Configures the EZSP stack and starts the high-level NCP services.
    ///
    /// The startup sequence applies configured policies and stack values,
    /// waits for protocol negotiation, applies its [`Startup`] mode, waits for
    /// `NetworkUp`, registers every supplied endpoint, starts callback
    /// translation and correlation, and returns an [`Ncp`] containing a
    /// cloneable [`Connected`] actor handle. Translated events are sent to
    /// `events`.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if endpoint validation, actor communication,
    /// protocol negotiation, stack setup, network initialization, or endpoint
    /// registration fails.
    pub async fn start<E>(
        self,
        startup: Startup,
        endpoints: Box<[Endpoint]>,
        events: Sender<E>,
    ) -> Result<Ncp<Connected>, Error>
    where
        E: TranslatableEvent + 'static,
    {
        if endpoints.is_empty() {
            return Err(Error::NoEndpoints);
        }

        let disconnected = self
            .transceiver
            .spawn(self.callbacks_capacity, self.messages_capacity);

        let (mut connected, mut callbacks) = disconnected.connect(self.desired_version).await?;

        debug!("Setting concentrator");
        connected.set_concentrator(self.concentrator).await?;

        for (key, value) in self.configuration {
            debug!("Setting configuration {key:?} to {value:#06X}");
            connected.set_configuration_value(key, value).await?;
        }

        for (key, value) in self.policy {
            debug!("Setting policy {key:?} to {value:#04X}");
            connected.set_policy(key, value).await?;
        }

        let ieee_address = connected.get_eui64().await?;
        debug!("IEEE address: {ieee_address}");

        let network_state = connected.network_state().await?;
        info!("Current network state: {network_state:?}");

        let (message_tx, message_rx) = channel(self.messages_capacity);

        info!("Initializing NCP.");
        let ncp = Ncp::new(
            connected.clone(),
            endpoints,
            message_tx.clone(),
            self.aps_options,
        )
        .await?;

        match startup {
            Startup::Initialize(init) => {
                if connected.leave_network().await.is_ok() {
                    callbacks.await_network_down().await;
                    info!("Left existing network.");
                }

                debug!("Setting initial security state");
                connected
                    .set_initial_security_state(init.initial_security_state())
                    .await?;

                info!("Reinitializing network");
                connected
                    .form_network(init.parameters(self.radio_tx_power))
                    .await?;
            }
            Startup::Resume(init_bitmask) => {
                connected.network_init(init_bitmask).await?;
            }
        }

        callbacks.await_network_up().await;
        info!("Network is up.");

        debug!("Setting radio power to {}", self.radio_tx_power);
        connected.set_radio_power(self.radio_tx_power).await?;

        let network_state = connected.network_state().await?;
        info!("Final network state: {network_state:?}");

        let (typ, parameters) = connected.get_network_parameters().await?;
        info!("Device type: {typ}");
        info!("Network parameters:\n{parameters}");

        log_state(&mut connected).await?;

        let radius: u8 = connected
            .get_configuration_value(config::Id::MaxHops)
            .await?
            .try_into()
            .unwrap_or_default();
        info!("Sending many-to-one route request: {radius} hops");
        connected
            .send_many_to_one_route_request(concentrator::Type::HighRam, radius)
            .await?;

        info!("Starting message translation bridge.");
        spawn(async move {
            while let Some(callback) = callbacks.recv().await {
                if let Err(error) = message_tx.send(callback.into()).await {
                    info!("Message handler has closed. Terminating bridge.");
                    trace!("{error}");
                    break;
                }
            }
        });

        info!("Starting event handler.");
        spawn(EventHandler::new(connected, events).run(message_rx));

        Ok(ncp)
    }
}

async fn log_state<T>(transport: &mut T) -> Result<(), Error>
where
    T: Configuration + Security + Send,
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
