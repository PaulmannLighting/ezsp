use silizium::zigbee::security::man::Key;

use crate::ember::join::Method;
use crate::ember::network::Parameters;
use crate::ember::security::initial::{Bitmask, State};
use crate::ncp::network_credentials::NetworkCredentials;

const DEFAULT_NWK_MANAGER_ID: u16 = 0x0000;
const DEFAULT_NWK_UPDATE_ID: u8 = 0x00;
const CHANNEL_BIT: u32 = 1;
const NETWORK_KEY_SEQUENCE_NUMBER: u8 = 0;

/// Parameters used to leave any current network and form a new Zigbee network.
///
/// Wrap this value in [`Startup::Initialize`](crate::Startup::Initialize) and
/// pass it to a [`Builder`](crate::Builder) constructor to select explicit
/// network formation. Use [`Startup::Resume`](crate::Startup::Resume) instead
/// when the NCP should restore its persisted network state.
///
/// Network identity and network-key material are grouped in
/// [`NetworkCredentials`](NetworkCredentials). The remaining values are
/// specific to forming the network: the preconfigured link key, radio channel,
/// and join method.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InitializationParameters {
    network_credentials: NetworkCredentials,
    link_key: Key,
    radio_channel: u8,
    join_method: Method,
    nwk_manager_id: u16,
    nwk_update_id: u8,
}

impl InitializationParameters {
    /// Creates explicit network formation parameters.
    ///
    /// The credential network key and trust-center EUI-64 are installed in the
    /// initial security state. `link_key` becomes the preconfigured trust-center
    /// link key. The network manager ID and network update ID initially use
    /// their default zero values.
    ///
    /// Zigbee radio channels normally range from 11 through 26. This function
    /// does not validate `radio_channel`.
    #[must_use]
    pub const fn new(
        network_credentials: NetworkCredentials,
        link_key: Key,
        radio_channel: u8,
        join_method: Method,
    ) -> Self {
        Self {
            network_credentials,
            link_key,
            radio_channel,
            join_method,
            nwk_manager_id: DEFAULT_NWK_MANAGER_ID,
            nwk_update_id: DEFAULT_NWK_UPDATE_ID,
        }
    }

    /// Creates the Ember initial security state for network formation.
    ///
    /// The returned state contains both the network key from the credentials
    /// and the separately supplied preconfigured link key.
    #[must_use]
    pub fn initial_security_state(&self) -> State {
        State::new(
            Bitmask::TRUST_CENTER_GLOBAL_LINK_KEY
                | Bitmask::HAVE_PRECONFIGURED_KEY
                | Bitmask::REQUIRE_ENCRYPTED_KEY
                | Bitmask::HAVE_NETWORK_KEY
                | Bitmask::HAVE_TRUST_CENTER_EUI64,
            self.link_key,
            self.network_credentials.network_key,
            NETWORK_KEY_SEQUENCE_NUMBER,
            self.network_credentials.trust_center_eui64,
        )
    }

    /// Returns a channel mask containing the configured radio channel.
    ///
    /// # Panics
    ///
    /// Panics if the configured channel is not representable in a 32-bit
    /// channel mask.
    #[must_use]
    pub const fn channels(&self) -> u32 {
        CHANNEL_BIT << self.radio_channel
    }

    /// Creates the Ember network parameters used to form the network.
    ///
    /// The network identifiers come from the credentials. `radio_tx_power` is
    /// supplied separately because it remains configurable on the builder.
    ///
    /// # Panics
    ///
    /// Panics if the configured channel is not representable in a 32-bit
    /// channel mask.
    #[must_use]
    pub fn parameters(&self, radio_tx_power: i8) -> Parameters {
        Parameters::new(
            self.network_credentials.extended_pan_id,
            self.network_credentials.pan_id,
            radio_tx_power,
            self.radio_channel,
            self.join_method,
            self.nwk_manager_id,
            self.nwk_update_id,
            self.channels(),
        )
    }
}
