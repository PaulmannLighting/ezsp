use std::collections::BTreeMap;

use macaddr::MacAddr8;
use silizium::zigbee::security::man::Key;
use tokio::sync::mpsc::Receiver;

use crate::Callback;
use crate::ember::{aps, concentrator, join};
use crate::ezsp::network::InitBitmask;
use crate::ezsp::{config, policy};

const RADIO_CHANNEL: u8 = 11;
const RADIO_POWER: i8 = 8;

/// Builder for Zigbee device configuration.
pub struct Builder<T> {
    pub(crate) transport: T,
    pub(crate) callbacks: Receiver<Callback>,
    pub(crate) policy: BTreeMap<policy::Id, u8>,
    pub(crate) configuration: BTreeMap<config::Id, u16>,
    pub(crate) concentrator: Option<concentrator::Parameters>,
    pub(crate) init_bitmask: InitBitmask,
    pub(crate) aps_options: aps::Options,
    pub(crate) link_key: Option<Key>,
    pub(crate) network_key: Option<Key>,
    pub(crate) join_method: join::Method,
    pub(crate) pan_id: Option<u16>,
    pub(crate) ieee_address: Option<MacAddr8>,
    pub(crate) radio_channel: u8,
    pub(crate) radio_power: i8,
    pub(crate) reinitialize: bool,
    pub(crate) buffers: usize,
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

#[cfg(feature = "ashv2")]
impl Builder<crate::uart::Uart> {
    /// Create a new builder using an `ASHv2` UART on the given serial port.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the serial port cannot be used for `ASHv2` communication.
    pub fn ashv2<T>(serial_port: T) -> Result<Self, crate::Error>
    where
        T: ashv2::SerialPort + ashv2::TryCloneNative + Sync + 'static,
    {
        Self::ashv2_with_buffers(serial_port, &crate::uart::Buffers::default())
    }

    /// Create a new builder using an `ASHv2` UART on the given serial port.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the serial port cannot be used for `ASHv2` communication.
    pub fn ashv2_with_buffers<T>(
        serial_port: T,
        buffers: &crate::uart::Buffers,
    ) -> Result<Self, crate::Error>
    where
        T: ashv2::SerialPort + ashv2::TryCloneNative + Sync + 'static,
    {
        let (ash_tx, ash_rx) = tokio::sync::mpsc::channel(buffers.ash_receiver);
        let (_ashv2_tasks, proxy) = ashv2::Actor::new(serial_port, ash_tx, buffers.ash_transmitter)
            .map_err(std::io::Error::from)?
            .spawn();
        let (callbacks_tx, callbacks_rx) = tokio::sync::mpsc::channel(buffers.ezsp_callbacks);
        let uart = crate::uart::Uart::new(
            proxy,
            ash_rx,
            callbacks_tx,
            crate::MIN_NON_LEGACY_VERSION,
            buffers.ezsp_messages,
        );
        Ok(Self::new(uart, callbacks_rx))
    }
}
