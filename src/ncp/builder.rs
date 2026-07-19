use std::collections::BTreeMap;

use tokio::sync::mpsc::Receiver;

use crate::ember::{aps, concentrator};
use crate::ezsp::{config, policy};
use crate::{Callback, Startup};

const RADIO_POWER: i8 = 8;

/// Builder for [`Ncp`](crate::Ncp) startup configuration.
///
/// The builder stores the selected [`Startup`] mode, EZSP policies, stack
/// configuration values, radio settings, and callback buffers until a
/// feature-specific startup implementation consumes it.
#[cfg_attr(not(feature = "apis-saltans"), expect(dead_code))]
pub struct Builder<T> {
    pub(crate) transport: T,
    pub(crate) callbacks: Receiver<Callback>,
    pub(crate) startup: Startup,
    pub(crate) policy: BTreeMap<policy::Id, u8>,
    pub(crate) configuration: BTreeMap<config::Id, u16>,
    pub(crate) concentrator: Option<concentrator::Parameters>,
    pub(crate) radio_tx_power: i8,
    pub(crate) aps_options: aps::Options,
    pub(crate) buffers: usize,
}

impl<T> Builder<T> {
    /// Creates a builder with a transport, callback stream, and startup mode.
    ///
    /// Select [`Startup::Resume`] to restore a persisted network or
    /// [`Startup::Initialize`] to leave any current network and form a new one.
    #[must_use]
    pub const fn new(transport: T, callbacks: Receiver<Callback>, startup: Startup) -> Self {
        Self {
            transport,
            callbacks,
            startup,
            policy: BTreeMap::new(),
            configuration: BTreeMap::new(),
            concentrator: None,
            radio_tx_power: RADIO_POWER,
            aps_options: aps::Options::empty(),
            buffers: 1024,
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
}

#[cfg(feature = "ashv2")]
impl Builder<crate::uart::Uart> {
    /// Creates a new builder using an `ASHv2` UART on the given serial port.
    ///
    /// The serial port must implement [`crate::uart::SerialPort`]. The returned
    /// [`crate::uart::Futures`] value must be spawned or otherwise polled by the
    /// caller for the UART transport to make progress. `startup` selects
    /// whether the NCP restores its persisted network or forms a new one.
    pub fn ashv2<T>(serial_port: T, startup: Startup) -> (Self, crate::uart::Futures<T>)
    where
        T: crate::uart::SerialPort + Sync + 'static,
    {
        Self::ashv2_with_buffers(serial_port, startup, crate::uart::Buffers::default())
    }

    /// Creates a new builder using an `ASHv2` UART on the given serial port.
    ///
    /// The serial port must implement [`crate::uart::SerialPort`]. Use
    /// [`crate::uart::Buffers`] to size the EZSP and `ASHv2` channels used by the
    /// constructed transport. The returned [`crate::uart::Futures`] value must
    /// be spawned or otherwise polled by the caller for the UART transport to
    /// make progress. `startup` selects whether the NCP restores its persisted
    /// network or forms a new one.
    pub fn ashv2_with_buffers<T>(
        serial_port: T,
        startup: Startup,
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
            Self::new(uart, callbacks_rx, startup),
            crate::uart::Futures::new(splitter, futures),
        )
    }
}
