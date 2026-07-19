use std::collections::BTreeMap;

use tokio::sync::mpsc::Receiver;

use crate::ember::{aps, concentrator};
use crate::ezsp::network::InitBitmask;
use crate::ezsp::{config, policy};
use crate::{Callback, InitializationParameters};

const RADIO_POWER: i8 = 8;

/// Builder for [`Ncp`](crate::Ncp) startup configuration.
///
/// The builder stores EZSP policies, stack configuration values, optional
/// network initialization parameters, radio settings, and callback buffers
/// until a feature-specific startup implementation consumes it.
#[cfg_attr(not(feature = "apis-saltans"), expect(dead_code))]
pub struct Builder<T> {
    pub(crate) transport: T,
    pub(crate) callbacks: Receiver<Callback>,
    pub(crate) policy: BTreeMap<policy::Id, u8>,
    pub(crate) configuration: BTreeMap<config::Id, u16>,
    pub(crate) concentrator: Option<concentrator::Parameters>,
    pub(crate) radio_tx_power: i8,
    pub(crate) init_bitmask: InitBitmask,
    pub(crate) aps_options: aps::Options,
    pub(crate) buffers: usize,
    pub(crate) initialization_parameters: Option<InitializationParameters>,
}

impl<T> Builder<T> {
    /// Creates a new builder with the given transport and callback stream.
    #[must_use]
    pub const fn new(transport: T, callbacks: Receiver<Callback>) -> Self {
        Self {
            transport,
            callbacks,
            policy: BTreeMap::new(),
            configuration: BTreeMap::new(),
            concentrator: None,
            radio_tx_power: RADIO_POWER,
            init_bitmask: InitBitmask::NO_OPTIONS,
            aps_options: aps::Options::empty(),
            buffers: 1024,
            initialization_parameters: None,
        }
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

    /// Sets the default APS options for outgoing APS messages created by [`Ncp`](crate::Ncp).
    #[must_use]
    pub const fn with_aps_options(mut self, options: aps::Options) -> Self {
        self.aps_options = options;
        self
    }

    /// Sets the buffer size used for callback, event, and NCP actor channels.
    #[must_use]
    pub const fn with_buffers(mut self, buffers: usize) -> Self {
        self.buffers = buffers;
        self
    }

    /// Selects explicit network initialization for startup.
    ///
    /// When configured, startup leaves any current network, applies the
    /// supplied security state, and forms the requested network. Without this
    /// call, startup resumes persisted network state through `networkInit`.
    ///
    /// # Errors
    ///
    /// Returns the supplied parameters if initialization was already selected.
    pub fn initialize(
        mut self,
        initialization_parameters: InitializationParameters,
    ) -> Result<Self, InitializationParameters> {
        if self.initialization_parameters.is_some() {
            Err(initialization_parameters)
        } else {
            self.initialization_parameters
                .replace(initialization_parameters);
            Ok(self)
        }
    }
}

#[cfg(feature = "ashv2")]
impl Builder<crate::uart::Uart> {
    /// Create a new builder using an `ASHv2` UART on the given serial port.
    ///
    /// The serial port must implement [`crate::uart::SerialPort`]. The returned
    /// [`crate::uart::Futures`] value must be spawned or otherwise polled by the
    /// caller for the UART transport to make progress.
    pub fn ashv2<T>(serial_port: T) -> (Self, crate::uart::Futures<T>)
    where
        T: crate::uart::SerialPort + Sync + 'static,
    {
        Self::ashv2_with_buffers(serial_port, crate::uart::Buffers::default())
    }

    /// Create a new builder using an `ASHv2` UART on the given serial port.
    ///
    /// The serial port must implement [`crate::uart::SerialPort`]. Use
    /// [`crate::uart::Buffers`] to size the EZSP and `ASHv2` channels used by the
    /// constructed transport. The returned [`crate::uart::Futures`] value must
    /// be spawned or otherwise polled by the caller for the UART transport to
    /// make progress.
    pub fn ashv2_with_buffers<T>(
        serial_port: T,
        buffers: crate::uart::Buffers,
    ) -> (Self, crate::uart::Futures<T>)
    where
        T: crate::uart::SerialPort + Sync + 'static,
    {
        let (ash_tx, ash_rx) = tokio::sync::mpsc::channel(buffers.ash_receiver);
        let (ash_v2, futures) = crate::uart::start(serial_port, ash_tx);
        let (callbacks_tx, callbacks_rx) = tokio::sync::mpsc::channel(buffers.ezsp_callbacks);
        let (uart, splitter) = crate::uart::Uart::new(
            ash_v2,
            ash_rx,
            callbacks_tx,
            crate::MIN_NON_LEGACY_VERSION,
            buffers.ezsp_messages,
        );
        (
            Self::new(uart, callbacks_rx),
            crate::uart::Futures::new(splitter, futures),
        )
    }
}

#[cfg(test)]
mod tests {
    use silizium::zigbee::security::man::Key;
    use tokio::sync::mpsc::channel;

    use super::*;
    use crate::ember::join::Method;
    use crate::ember::{Eui64, PanId};

    const EUI64_LENGTH: usize = 8;
    const FIRST_ADDRESS_BYTE: u8 = 0x02;
    const KEY_LENGTH: usize = 16;
    const SECOND_ADDRESS_BYTE: u8 = 0x04;
    const CHANNEL_SIZE: usize = 1;
    const LINK_KEY: Key = [0x22; KEY_LENGTH];
    const NETWORK_KEY: Key = [0x11; KEY_LENGTH];
    const PAN_ID: PanId = 0x1234;
    const RADIO_CHANNEL: u8 = 15;

    #[test]
    fn initialize_rejects_reconfiguration() {
        let (_sender, receiver) = channel(CHANNEL_SIZE);
        let first = initialization_parameters(FIRST_ADDRESS_BYTE);
        let second = initialization_parameters(SECOND_ADDRESS_BYTE);
        let Ok(builder) = Builder::new((), receiver).initialize(first) else {
            panic!("first initialization should succeed");
        };
        let Err(returned) = builder.initialize(second) else {
            panic!("second initialization should fail");
        };

        assert_eq!(returned, second);
    }

    fn initialization_parameters(address_byte: u8) -> InitializationParameters {
        InitializationParameters::new(
            Eui64::from([address_byte; EUI64_LENGTH]),
            PAN_ID,
            Eui64::from([address_byte; EUI64_LENGTH]),
            NETWORK_KEY,
            LINK_KEY,
            RADIO_CHANNEL,
            Method::ConfiguredNwkState,
        )
    }
}
