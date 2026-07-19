use std::io::ErrorKind;
use std::sync::Arc;
use std::time::Duration;

use apis_saltans_hw::zdp::SimpleDescriptor;
use apis_saltans_hw::{Driver, Event, EventTranslator, NcpHandle, bridge};
use log::{debug, info};
use tokio::spawn;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{Receiver, channel};
use tokio::time::sleep;

use super::event_handler::EventHandler;
use crate::ember::concentrator;
use crate::ezsp::config;
use crate::{
    Builder, Communicate, Configuration, ConfigurationExt, Displayable, Error, Messaging, Ncp,
    Networking, PolicyExt, Security, Startup, Utilities,
};

const TICK: Duration = Duration::from_secs(1);

impl<T> Builder<T>
where
    T: Communicate,
{
    /// Connects the underlying transport before startup configuration begins.
    ///
    /// `ASHv2` transports may report [`ErrorKind::NotConnected`] while their
    /// caller-owned futures are still establishing the serial link. In that
    /// case this helper waits for [`TICK`] and retries. Any other connection
    /// error is returned immediately.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the transport reports a connection error other
    /// than [`ErrorKind::NotConnected`].
    async fn connect(&mut self) -> Result<(), Error> {
        loop {
            let Err(error) = self.transport.ensure_connection().await else {
                return Ok(());
            };

            if let Error::Io(io_error) = &error
                && io_error.kind() == ErrorKind::NotConnected
            {
                debug!("ASHv2 is not yet connected. Waiting...");
                sleep(TICK).await;
            } else {
                return Err(error);
            }
        }
    }
}

impl<T> Builder<T>
where
    T: Communicate
        + Configuration
        + Security
        + Messaging
        + Networking
        + Utilities
        + Send
        + Sync
        + 'static,
{
    /// Configures the EZSP stack and starts an `apis_saltans_hw` NCP actor.
    ///
    /// The startup sequence applies configured policies and stack values,
    /// waits for the transport to connect, registers the supplied endpoints,
    /// stores their cluster lists for later APS source endpoint selection, and
    /// applies the [`Startup`] mode supplied to the builder. It then waits for
    /// `NetworkUp`, sends a many-to-one route request, and returns an
    /// `apis_saltans_hw::NcpHandle` plus the translated event stream.
    ///
    /// # Errors
    ///
    /// Returns an `apis_saltans_hw::Error` if endpoint validation, EZSP stack
    /// setup, transport connection, network initialization, or actor startup
    /// fails.
    pub async fn start(
        mut self,
        endpoints: &[SimpleDescriptor],
    ) -> Result<(NcpHandle, Receiver<Event>), apis_saltans_hw::Error> {
        if endpoints.is_empty() {
            return Err(apis_saltans_hw::Error::NoEndpoints);
        }

        self.connect().await?;
        let mut transport = Arc::new(Mutex::new(self.transport));

        let (message_tx, message_rx) = channel(self.buffers);
        spawn(bridge(self.callbacks, message_tx.clone()));
        let (events_tx, mut events_rx) = channel(self.buffers);
        spawn(EventHandler::new(events_tx, transport.clone()).run(message_rx));

        debug!("Setting concentrator");
        transport.set_concentrator(self.concentrator).await?;

        for (key, value) in self.configuration {
            debug!("Setting configuration {key:?} to {value:#06X}");
            transport.set_configuration_value(key, value).await?;
        }

        for (key, value) in self.policy {
            debug!("Setting policy {key:?} to {value:#04X}");
            transport.set_policy(key, value).await?;
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
            transport
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

        let ieee_address = transport.get_eui64().await?;
        debug!("IEEE address: {ieee_address}");

        let network_state = transport.network_state().await?;
        info!("Current network state: {network_state:?}");

        match self.startup {
            Startup::Initialize(init) => {
                if transport.leave_network().await.is_ok() {
                    wait_for_event(&mut events_rx, Event::NetworkDown).await?;
                    info!("Left existing network.");
                }

                debug!("Setting initial security state");
                transport
                    .set_initial_security_state(init.initial_security_state())
                    .await?;

                info!("Reinitializing network");
                transport
                    .form_network(init.parameters(self.radio_tx_power))
                    .await?;
            }
            Startup::Resume(init_bitmask) => {
                transport.network_init(init_bitmask).await?;
            }
        }

        wait_for_event(&mut events_rx, Event::NetworkUp).await?;
        info!("Network is up.");

        debug!("Setting radio power to {}", self.radio_tx_power);
        transport.set_radio_power(self.radio_tx_power).await?;

        let network_state = transport.network_state().await?;
        info!("Final network state: {network_state:?}");

        let (typ, parameters) = transport.get_network_parameters().await?;
        info!("Device type: {typ}");
        info!("Network parameters:\n{parameters}");

        log_state(&mut transport).await?;

        let radius = transport
            .get_configuration_value(config::Id::MaxHops)
            .await?;
        info!("Sending many-to-one route request: {radius} hops");
        #[expect(clippy::cast_possible_truncation)]
        transport
            .send_many_to_one_route_request(concentrator::Type::HighRam, radius as u8)
            .await?;

        let (ncp, actor) = Ncp::new(
            transport,
            self.aps_options,
            message_tx,
            endpoints.iter().map(Into::into).collect(),
        )
        .run(32);
        spawn(actor);
        Ok((ncp, events_rx))
    }
}

async fn wait_for_event(
    events: &mut Receiver<Event>,
    expected: Event,
) -> Result<(), apis_saltans_hw::Error> {
    while let Some(event) = events.recv().await {
        if event == expected {
            return Ok(());
        }

        debug!("Ignoring event while waiting for {expected:?}: {event:?}");
    }

    Err(Error::ChannelClosed.into())
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

fn implementation_error<T>(error: T) -> apis_saltans_hw::Error
where
    T: std::error::Error + Send + Sync + 'static,
{
    apis_saltans_hw::Error::Implementation(Arc::new(error))
}
