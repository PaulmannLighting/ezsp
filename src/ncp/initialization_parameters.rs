use std::ops::RangeInclusive;

use rand::{CryptoRng, Rng, RngExt};
use silizium::zigbee::security::man::Key;

use crate::ember::join::Method;
#[cfg(feature = "apis-saltans")]
use crate::ember::network::Parameters;
#[cfg(feature = "apis-saltans")]
use crate::ember::security::initial::{Bitmask, State};
use crate::ember::{Eui64, PanId};

const DEFAULT_NWK_MANAGER_ID: u16 = 0x0000;
const DEFAULT_NWK_UPDATE_ID: u8 = 0x00;
#[cfg(feature = "apis-saltans")]
const CHANNEL_BIT: u32 = 1;
const EUI64_LENGTH: usize = 8;
const LOCAL_BIT: u8 = 0x02;
const MULTICAST_BIT: u8 = 0x01;
#[cfg(feature = "apis-saltans")]
const NETWORK_KEY_SEQUENCE_NUMBER: u8 = 0;
const VALID_PAN_IDS: RangeInclusive<u16> = 0x0000..=0xFFFE;

/// Parameters used to leave any current network and form a new Zigbee network.
///
/// Pass this value to [`Builder::initialize`](crate::Builder::initialize) to
/// select explicit network formation during startup. Without initialization
/// parameters, [`Builder::start`](crate::Builder::start) resumes persisted
/// network state through `networkInit` instead.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InitializationParameters {
    pub(crate) extended_pan_id: Eui64,
    pub(crate) pan_id: PanId,
    pub(crate) trust_center_eui64: Eui64,
    pub(crate) network_key: Key,
    pub(crate) link_key: Key,
    pub(crate) radio_channel: u8,
    pub(crate) join_method: Method,
    pub(crate) nwk_manager_id: u16,
    pub(crate) nwk_update_id: u8,
}

impl InitializationParameters {
    /// Creates explicit network formation parameters.
    ///
    /// `extended_pan_id` identifies the new network, while
    /// `trust_center_eui64` is installed as the preconfigured trust-center
    /// address. The network manager ID and network update ID initially use
    /// their default zero values.
    #[must_use]
    pub const fn new(
        extended_pan_id: Eui64,
        pan_id: PanId,
        trust_center_eui64: Eui64,
        network_key: Key,
        link_key: Key,
        radio_channel: u8,
        join_method: Method,
    ) -> Self {
        Self {
            extended_pan_id,
            pan_id,
            trust_center_eui64,
            network_key,
            link_key,
            radio_channel,
            join_method,
            nwk_manager_id: DEFAULT_NWK_MANAGER_ID,
            nwk_update_id: DEFAULT_NWK_UPDATE_ID,
        }
    }

    /// Creates parameters with random network identifiers and a random network key.
    ///
    /// The generated extended PAN ID and trust-center EUI-64 are locally
    /// administered unicast addresses. The PAN ID excludes the reserved
    /// broadcast value `0xFFFF`. The supplied link key, radio channel, and join
    /// method are preserved.
    pub fn random<T>(rng: &mut T, link_key: Key, radio_channel: u8, join_method: Method) -> Self
    where
        T: CryptoRng,
    {
        Self::new(
            random_mac_addr8(rng),
            rng.random_range(VALID_PAN_IDS),
            random_mac_addr8(rng),
            rng.random(),
            link_key,
            radio_channel,
            join_method,
        )
    }

    #[cfg(feature = "apis-saltans")]
    #[must_use]
    pub(crate) fn initial_security_state(&self) -> State {
        State::new(
            Bitmask::TRUST_CENTER_GLOBAL_LINK_KEY
                | Bitmask::HAVE_PRECONFIGURED_KEY
                | Bitmask::REQUIRE_ENCRYPTED_KEY
                | Bitmask::HAVE_NETWORK_KEY
                | Bitmask::HAVE_TRUST_CENTER_EUI64,
            self.link_key,
            self.network_key,
            NETWORK_KEY_SEQUENCE_NUMBER,
            self.trust_center_eui64,
        )
    }

    #[cfg(feature = "apis-saltans")]
    #[must_use]
    pub(crate) const fn channels(&self) -> u32 {
        CHANNEL_BIT << self.radio_channel
    }

    #[cfg(feature = "apis-saltans")]
    #[must_use]
    pub(crate) fn parameters(&self, radio_tx_power: i8) -> Parameters {
        Parameters::new(
            self.extended_pan_id,
            self.pan_id,
            radio_tx_power,
            self.radio_channel,
            self.join_method,
            self.nwk_manager_id,
            self.nwk_update_id,
            self.channels(),
        )
    }
}

fn random_mac_addr8<T>(rng: &mut T) -> Eui64
where
    T: Rng,
{
    loop {
        let mut bytes = [0u8; EUI64_LENGTH];
        rng.fill(&mut bytes);

        // IEEE EUI-64:
        // - clear multicast bit (bit 0)
        // - set locally administered bit (bit 1)
        bytes[0] = (bytes[0] & !MULTICAST_BIT) | LOCAL_BIT;

        // Avoid common sentinel values.
        if bytes != [0; EUI64_LENGTH] && bytes != [u8::MAX; EUI64_LENGTH] {
            return Eui64::from(bytes);
        }
    }
}

#[cfg(all(test, feature = "apis-saltans"))]
mod tests {
    use super::*;

    const KEY_LENGTH: usize = 16;
    const EXTENDED_PAN_ID: [u8; EUI64_LENGTH] = [0x02; EUI64_LENGTH];
    const LINK_KEY: Key = [0x22; KEY_LENGTH];
    const NETWORK_KEY: Key = [0x11; KEY_LENGTH];
    const PAN_ID: PanId = 0x1234;
    const RADIO_CHANNEL: u8 = 15;
    const RADIO_TX_POWER: i8 = -3;
    const TRUST_CENTER_EUI64: [u8; EUI64_LENGTH] = [0x04; EUI64_LENGTH];

    #[test]
    fn creates_initial_security_state() {
        let initialization_parameters = initialization_parameters();
        let state = initialization_parameters.initial_security_state();
        let expected_bitmask = Bitmask::TRUST_CENTER_GLOBAL_LINK_KEY
            | Bitmask::HAVE_PRECONFIGURED_KEY
            | Bitmask::REQUIRE_ENCRYPTED_KEY
            | Bitmask::HAVE_NETWORK_KEY
            | Bitmask::HAVE_TRUST_CENTER_EUI64;

        assert_eq!(state.bitmask(), expected_bitmask);
        assert_eq!(state.preconfigured_key(), &LINK_KEY);
        assert_eq!(state.network_key(), &NETWORK_KEY);
        assert_eq!(
            state.network_key_sequence_number(),
            NETWORK_KEY_SEQUENCE_NUMBER
        );
        assert_eq!(
            state.preconfigured_trust_center_eui64(),
            TRUST_CENTER_EUI64.into()
        );
    }

    #[test]
    fn creates_network_parameters() {
        let initialization_parameters = initialization_parameters();
        let parameters = initialization_parameters.parameters(RADIO_TX_POWER);

        assert_eq!(parameters.extended_pan_id(), EXTENDED_PAN_ID.into());
        assert_eq!(parameters.pan_id(), PAN_ID);
        assert_eq!(parameters.radio_tx_power(), RADIO_TX_POWER);
        assert_eq!(parameters.radio_channel(), RADIO_CHANNEL);
        assert_eq!(parameters.join_method(), Some(Method::ConfiguredNwkState));
        assert_eq!(parameters.nwk_manager_id(), DEFAULT_NWK_MANAGER_ID);
        assert_eq!(parameters.nwk_update_id(), DEFAULT_NWK_UPDATE_ID);
        assert_eq!(parameters.channels(), CHANNEL_BIT << RADIO_CHANNEL);
    }

    fn initialization_parameters() -> InitializationParameters {
        InitializationParameters::new(
            EXTENDED_PAN_ID.into(),
            PAN_ID,
            TRUST_CENTER_EUI64.into(),
            NETWORK_KEY,
            LINK_KEY,
            RADIO_CHANNEL,
            Method::ConfiguredNwkState,
        )
    }
}
