use crate::InitializationParameters;
use crate::ezsp::network::InitBitmask;

/// Selects how an [`Ncp`](crate::Ncp) establishes its network during startup.
///
/// Supply a startup mode when constructing a [`Builder`](crate::Builder). Use
/// [`Startup::Resume`] for normal restarts that should restore the network
/// persisted by the NCP. Use [`Startup::Initialize`] only when the application
/// intends to leave any current network and form the network described by the
/// supplied [`InitializationParameters`].
///
/// The startup mode is consumed by [`Builder::start`](crate::Builder::start)
/// after the transport, stack configuration, and endpoints have been set up.
#[derive(Debug)]
pub enum Startup {
    /// Leaves any current network, installs the initial security state, and
    /// forms a network with the supplied parameters.
    ///
    /// This replaces the NCP's current network configuration. The radio
    /// transmit power comes from [`Builder::with_radio_tx_power`](crate::Builder::with_radio_tx_power),
    /// rather than from [`InitializationParameters`].
    Initialize(InitializationParameters),

    /// Restores the NCP's persisted network state through `networkInit`.
    ///
    /// [`InitBitmask::NO_OPTIONS`] is the normal choice when no end-device
    /// parent restoration or rejoin behavior is required.
    Resume(InitBitmask),
}
