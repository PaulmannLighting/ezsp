use std::collections::BTreeMap;
use std::num::NonZero;

use log::{debug, info, trace};
use tokio::sync::mpsc::{Receiver, Sender, channel};

pub use self::build_result::BuildResult;
use crate::ember::aps::Options;
use crate::ember::{self, concentrator, network};
use crate::ezsp::{config, policy};
use crate::ncp::await_event::AwaitEvent;
use crate::ncp::{DEFAULT_RADIO_TX_POWER, Endpoint};
use crate::{
    Callback, Client, Configuration, ConfigurationExt, Connection, Displayable, Error,
    EventHandler, MIN_NON_LEGACY_VERSION, Messaging, Ncp, Networking, PolicyExt, Security, Startup,
    TranslatableEvent, Utilities, ValueError,
};

mod build_result;

const EVENT_MESSAGES_CAPACITY: usize = 64;

/// Configures an actor-backed EZSP Network Co-Processor.
///
/// The builder owns a pre-negotiation [`Client`] handle. Create that handle
/// with [`Client::run`] and spawn both returned transport-actor futures before
/// [`Builder::start`] is awaited. The builder also stores the
/// event-message queue capacity, desired EZSP version, stack policies and
/// configuration values, concentrator settings, radio power, and baseline APS
/// route-discovery and address options. [`Ncp`] combines those baseline options
/// with the options supplied to each outgoing send.
pub struct Builder {
    pub(crate) client: Client,
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
    /// Creates a builder from a pre-negotiation client.
    ///
    /// The requested protocol version defaults to [`MIN_NON_LEGACY_VERSION`].
    #[must_use]
    pub const fn new(client: Client) -> Self {
        Self {
            client,
            event_messages_capacity: EVENT_MESSAGES_CAPACITY,
            desired_version: MIN_NON_LEGACY_VERSION,
            policy: BTreeMap::new(),
            configuration: BTreeMap::new(),
            concentrator: None,
            radio_tx_power: DEFAULT_RADIO_TX_POWER,
            manufacturer_code: None,
            options: Options::NONE,
        }
    }

    /// Sets the capacity of the channel between the callback bridge and event handler.
    ///
    /// Transport actor and callback channel capacities are selected through
    /// [`Client::run`].
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
    #[must_use]
    pub fn enable_route_discovery(mut self) -> Self {
        self.options.insert(Options::ENABLE_ROUTE_DISCOVERY);
        self
    }

    /// Forces route discovery for every outgoing frame, even when a route is known.
    #[must_use]
    pub fn force_route_discovery(mut self) -> Self {
        self.options.insert(Options::FORCE_ROUTE_DISCOVERY);
        self
    }

    /// Includes the source EUI-64 in every outgoing network frame.
    #[must_use]
    pub fn source_eui64(mut self) -> Self {
        self.options.insert(Options::SOURCE_EUI64);
        self
    }

    /// Includes the destination EUI-64 in every outgoing network frame.
    #[must_use]
    pub fn destination_eui64(mut self) -> Self {
        self.options.insert(Options::DESTINATION_EUI64);
        self
    }

    /// Enables ZDO address discovery when a destination node ID is unknown.
    #[must_use]
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
    /// start any lower-level transport tasks and spawn both futures returned by
    /// [`Client::run`]. After this method returns, spawn
    /// `BuildResult::bridge` before `BuildResult::event_handler`. Both must
    /// remain running while the NCP is in use so translated events can be sent
    /// to `events`.
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
        self.build(Some(startup), endpoints, events).await
    }

    /// Configure an idle NCP and register endpoints while leaving its network stopped.
    ///
    /// Use this to prepare a restore target. Run the returned bridge and event handler
    /// before submitting a restore through the hardware actor.
    ///
    /// # Errors
    /// Returns an error if the network is already running, or if setup fails.
    pub async fn prepare<E>(
        self,
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
        self.build(None, endpoints, events).await
    }

    async fn build<E>(
        self,
        startup: Option<Startup>,
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

        let (mut connected, mut callbacks) = self.client.connect(self.desired_version).await?;

        if startup.is_none() && connected.network_state().await? != network::Status::NoNetwork {
            return Err(ember::Status::InvalidCall.into());
        }

        configure_stack(
            &mut connected,
            self.concentrator,
            self.configuration,
            self.policy,
            self.manufacturer_code,
        )
        .await?;

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

        #[cfg(feature = "apis-saltans")]
        let ncp = Ncp {
            restore_tx_power: self.radio_tx_power,
            ..ncp
        };
        if let Some(startup) = startup {
            start_network(&mut connected, &mut callbacks, startup, self.radio_tx_power).await?;
            configure_running_network(&mut connected, self.radio_tx_power).await?;
        }

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

/// Applies stack settings in the required order before registering endpoints.
async fn configure_stack(
    connected: &mut Connection,
    concentrator: Option<concentrator::Parameters>,
    configuration: BTreeMap<config::Id, u16>,
    policy: BTreeMap<policy::Id, u8>,
    manufacturer_code: Option<u16>,
) -> Result<(), Error> {
    debug!("Setting concentrator");
    connected.set_concentrator(concentrator).await?;

    for (key, value) in configuration {
        debug!("Setting configuration {key:?} to {value:#06X}");
        connected.set_configuration_value(key, value).await?;
    }

    for (key, value) in policy {
        debug!("Setting policy {key:?} to {value:#04X}");
        connected.set_policy(key, value).await?;
    }

    if let Some(manufacturer_code) = manufacturer_code {
        debug!("Setting manufacturer code to {manufacturer_code:#06X}");
        connected.set_manufacturer_code(manufacturer_code).await?;
    }

    Ok(())
}

/// Forms or resumes a network and waits for its network-up callback.
async fn start_network(
    connected: &mut Connection,
    callbacks: &mut Receiver<Callback>,
    startup: Startup,
    radio_tx_power: i8,
) -> Result<(), Error> {
    match startup {
        Startup::Initialize(init) => {
            if connected.leave_network().await.is_ok() {
                callbacks.await_network_down().await?;
                info!("Left existing network.");
            }

            debug!("Setting initial security state");
            connected
                .set_initial_security_state(init.initial_security_state())
                .await?;

            info!("Reinitializing network");
            connected
                .form_network(init.parameters(radio_tx_power))
                .await?;
        }
        Startup::Resume(init_bitmask) => {
            connected.network_init(init_bitmask).await?;
        }
    }

    callbacks.await_network_up().await?;
    info!("Network is up.");

    Ok(())
}

async fn configure_running_network(
    connected: &mut Connection,
    radio_tx_power: i8,
) -> Result<(), Error> {
    debug!("Setting radio power to {radio_tx_power}");
    connected.set_radio_power(radio_tx_power).await?;

    let network_state = connected.network_state().await?;
    info!("Final network state: {network_state:?}");

    let (typ, parameters) = connected.get_network_parameters().await?;
    info!("Device type: {typ}");
    info!("Network parameters:\n{parameters}");

    log_state(connected).await?;

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

    Ok(())
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

#[cfg(all(test, feature = "apis-saltans"))]
mod tests {
    use std::num::NonZeroU8;

    use tokio::sync::mpsc;

    use crate::test_support::{ConnectionScript, Script, Transaction};
    use crate::types::ByteSizedVec;
    use crate::{Builder, Endpoint, Error, ember};

    const EZSP_V8: u8 = 8;
    const NETWORK_STATE: u16 = 0x0018;
    const SET_CONCENTRATOR: u16 = 0x0010;
    const GET_EUI64: u16 = 0x0026;
    const ADD_ENDPOINT: u16 = 0x0002;
    const ENDPOINT_ID: u8 = 1;
    const PROFILE_ID: u16 = 0x0104;
    const DEVICE_ID: u16 = 0x0005;
    const TX_POWER: i8 = 5;

    const fn endpoint() -> Endpoint {
        Endpoint {
            id: ENDPOINT_ID,
            profile_id: PROFILE_ID,
            device_id: DEVICE_ID,
            app_flags: 0,
            input_clusters: ByteSizedVec::new(),
            output_clusters: ByteSizedVec::new(),
        }
    }

    #[test]
    fn prepare_registers_endpoints_without_starting_the_network() {
        let (client, mut script) = ConnectionScript::client(
            EZSP_V8,
            Script::new([
                Transaction::reply(NETWORK_STATE, &[], &[0]),
                Transaction::reply(SET_CONCENTRATOR, &[0; 10], &[0]),
                Transaction::reply(GET_EUI64, &[], &[1; 8]),
                Transaction::reply(NETWORK_STATE, &[], &[0]),
                Transaction::reply(ADD_ENDPOINT, &[1, 4, 1, 5, 0, 0, 0, 0], &[0]),
            ]),
        );
        let (events, _incoming) = mpsc::channel::<apis_saltans_hw::Event<()>>(1);
        let builder = Builder::new(client)
            .with_desired_version(NonZeroU8::new(EZSP_V8).unwrap())
            .with_radio_tx_power(TX_POWER);
        let target = script
            .run(builder.prepare(Box::new([endpoint()]), events))
            .unwrap();
        assert_eq!(&*target.ncp.endpoints, &[endpoint()]);
        assert_eq!(target.ncp.restore_tx_power, TX_POWER);
    }

    #[test]
    fn prepare_rejects_a_running_network_before_configuration() {
        let (client, mut script) = ConnectionScript::client(
            EZSP_V8,
            Script::new([Transaction::reply(NETWORK_STATE, &[], &[2])]),
        );
        let (events, _incoming) = mpsc::channel::<apis_saltans_hw::Event<()>>(1);
        let builder = Builder::new(client).with_desired_version(NonZeroU8::new(EZSP_V8).unwrap());
        assert!(matches!(
            script.run(builder.prepare(Box::new([endpoint()]), events)),
            Err(Error::Status(crate::error::Status::Ember(Ok(
                ember::Status::InvalidCall
            ))))
        ));
    }
}
