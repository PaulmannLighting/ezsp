use std::sync::Arc;

use apis_saltans_hw::{AwaitEvent, Event, EventTranslator, NcpDriver, NcpHandle, Start, bridge};
use apis_saltans_zdp::SimpleDescriptor;
use log::{debug, info};
use macaddr::MacAddr8;
use rand::random;
use silizium::zigbee::security::man::Key;
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, channel};

use super::event_handler::EventHandler;
use crate::ember::security::initial;
use crate::ember::{concentrator, network};
use crate::ezsp::config;
use crate::{
    Builder, Configuration, ConfigurationExt, Displayable, Error, Messaging, Ncp, Networking,
    PolicyExt, Security, Transport, Utilities,
};

impl<T> Start for Builder<T>
where
    T: Transport + Sync + 'static,
{
    /// Starts the network manager on the given transport implementation.
    #[expect(clippy::too_many_lines)]
    async fn start(
        mut self,
        endpoints: &[SimpleDescriptor],
    ) -> Result<(NcpHandle, Receiver<Event>), apis_saltans_hw::Error> {
        if endpoints.is_empty() {
            return Err(apis_saltans_hw::Error::NoEndpoints);
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

        let (_handle, ncp) = Ncp::new(
            self.transport,
            // FIXME: Use first valid endpoint for now.
            endpoints
                .iter()
                .find_map(|endpoint| endpoint.profile().ok())
                .ok_or(apis_saltans_hw::Error::NoEndpoints)?
                .into(),
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

fn implementation_error<T>(error: T) -> apis_saltans_hw::Error
where
    T: std::error::Error + Send + Sync + 'static,
{
    apis_saltans_hw::Error::Implementation(Arc::new(error))
}
