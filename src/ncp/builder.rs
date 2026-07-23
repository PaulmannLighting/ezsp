use std::collections::BTreeMap;
use std::num::NonZero;

use log::{debug, info, trace};
use tokio::sync::mpsc::{Sender, channel};

pub use self::build_result::BuildResult;
use crate::ember::aps::Options;
use crate::ember::concentrator;
use crate::ezsp::{config, policy};
use crate::ncp::Endpoint;
use crate::ncp::await_event::AwaitEvent;
use crate::{
    Configuration, ConfigurationExt, Connectable, Displayable, Error, EventHandler,
    MIN_NON_LEGACY_VERSION, Messaging, Ncp, Networking, PolicyExt, Security, Startup,
    TranslatableEvent, Utilities, ValueError,
};

mod build_result;

const RADIO_POWER: i8 = 8;
const EVENT_MESSAGES_CAPACITY: usize = 64;

/// Configures an actor-backed EZSP Network Co-Processor.
///
/// The builder owns a pre-negotiation [`Connectable`] handle. The transport
/// futures that service this handle are returned separately by transport
/// constructors such as [`Builder::ashv2`](crate::Builder::ashv2) and must be
/// spawned by the caller before [`Builder::start`] is awaited. The builder also
/// stores the event-message queue capacity, desired EZSP version, stack
/// policies and configuration values, concentrator settings, radio power, and
/// baseline APS route-discovery and address options. [`Ncp`] combines those
/// baseline options with the options supplied to each outgoing send.
pub struct Builder {
    pub(crate) connectable: Connectable,
    pub(crate) event_messages_capacity: usize,
    pub(crate) desired_version: NonZero<u8>,
    pub(crate) policy: BTreeMap<policy::Id, u8>,
    pub(crate) configuration: BTreeMap<config::Id, u16>,
    pub(crate) concentrator: Option<concentrator::Parameters>,
    pub(crate) radio_tx_power: i8,
    pub(crate) manufacturer_code: Option<u16>,
    pub(crate) options: Options,
}

impl Builder {
    /// Creates a builder around separate transport transmit and receive halves.
    ///
    /// The requested protocol version defaults to [`MIN_NON_LEGACY_VERSION`].
    #[must_use]
    pub const fn new(connection: Connectable) -> Self {
        Self {
            connectable: connection,
            event_messages_capacity: EVENT_MESSAGES_CAPACITY,
            desired_version: MIN_NON_LEGACY_VERSION,
            policy: BTreeMap::new(),
            configuration: BTreeMap::new(),
            concentrator: None,
            radio_tx_power: RADIO_POWER,
            manufacturer_code: None,
            options: Options::NONE,
        }
    }

    /// Sets the capacity of the channel between the callback bridge and event handler.
    ///
    /// Transport actor and callback channel capacities are selected when the
    /// transport is constructed, for example with
    /// [`Builder::ashv2_with_buffers`](crate::Builder::ashv2_with_buffers).
    #[must_use]
    pub const fn with_event_messages_capacity(mut self, event_messages_capacity: usize) -> Self {
        self.event_messages_capacity = event_messages_capacity;
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

    /// Sets the manufacturer code in the NCP's node descriptor during startup.
    ///
    /// If this method is not called, the NCP's existing manufacturer code is
    /// left unchanged.
    #[must_use]
    pub const fn with_manufacturer_code(mut self, manufacturer_code: u16) -> Self {
        self.manufacturer_code = Some(manufacturer_code);
        self
    }

    /// Enables route discovery when an outgoing frame has no known route.
    pub fn enable_route_discovery(mut self) -> Self {
        self.options.insert(Options::ENABLE_ROUTE_DISCOVERY);
        self
    }

    /// Forces route discovery for every outgoing frame, even when a route is known.
    pub fn force_route_discovery(mut self) -> Self {
        self.options.insert(Options::FORCE_ROUTE_DISCOVERY);
        self
    }

    /// Includes the source EUI-64 in every outgoing network frame.
    pub fn source_eui64(mut self) -> Self {
        self.options.insert(Options::SOURCE_EUI64);
        self
    }

    /// Includes the destination EUI-64 in every outgoing network frame.
    pub fn destination_eui64(mut self) -> Self {
        self.options.insert(Options::DESTINATION_EUI64);
        self
    }

    /// Enables ZDO address discovery when a destination node ID is unknown.
    pub fn enable_address_discovery(mut self) -> Self {
        self.options.insert(Options::ENABLE_ADDRESS_DISCOVERY);
        self
    }
}

impl Builder {
    /// Configures the EZSP stack and creates the high-level NCP service futures.
    ///
    /// The startup sequence applies configured policies and stack values,
    /// waits for protocol negotiation, registers every supplied endpoint while
    /// the network is down, applies its [`Startup`] mode, waits for `NetworkUp`,
    /// and returns an [`Ncp`] containing a cloneable
    /// [`Connection`](crate::Connection) actor handle together with the
    /// callback bridge and event-handler futures.
    ///
    /// This method does not spawn tasks. Before awaiting it, the caller must
    /// spawn the transport futures in the order required by the transport. For
    /// [`Builder::ashv2`](crate::Builder::ashv2), that order is the `ASHv2` serial
    /// worker, `ASHv2` transmitter, `ASHv2` receiver, EZSP transmitter, then EZSP
    /// receiver. After this method returns, spawn `BuildResult::bridge` before
    /// `BuildResult::event_handler`. Both must remain running while the NCP is
    /// in use so translated events can be sent to `events`.
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
    ) -> Result<
        BuildResult<
            impl Future<Output = ()> + Send + 'static,
            impl Future<Output = ()> + Send + 'static,
        >,
        Error,
    >
    where
        E: TranslatableEvent + 'static,
    {
        if endpoints.is_empty() {
            return Err(Error::NoEndpoints);
        }

        let (mut connected, mut callbacks) = self.connectable.connect(self.desired_version).await?;

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

        if let Some(manufacturer_code) = self.manufacturer_code {
            debug!("Setting manufacturer code to {manufacturer_code:#06X}");
            connected.set_manufacturer_code(manufacturer_code).await?;
        }

        let ieee_address = connected.get_eui64().await?;
        debug!("IEEE address: {ieee_address}");

        let network_state = connected.network_state().await?;
        info!("Current network state: {network_state:?}");

        let (message_tx, message_rx) = channel(self.event_messages_capacity);

        info!("Initializing NCP.");
        let ncp = Ncp::new(
            connected.clone(),
            endpoints,
            message_tx.clone(),
            self.options,
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

        let configured_radius = connected
            .get_configuration_value(config::Id::MaxHops)
            .await?;
        let radius = configured_radius
            .try_into()
            .map_err(|_| ValueError::InvalidRouteRadius(configured_radius))?;
        info!("Sending many-to-one route request: {radius} hops");
        connected
            .send_many_to_one_route_request(concentrator::Type::HighRam, radius)
            .await?;

        info!("Creating message translation bridge.");
        let bridge = async move {
            while let Some(callback) = callbacks.recv().await {
                if let Err(error) = message_tx.send(callback.into()).await {
                    info!("Message handler has closed. Terminating bridge.");
                    trace!("{error}");
                    break;
                }
            }
        };

        info!("Creating event handler future.");
        let event_handler = EventHandler::new(connected, events).run(message_rx);

        Ok(BuildResult {
            ncp,
            bridge,
            event_handler,
        })
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
