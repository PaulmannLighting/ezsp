use std::ops::RangeInclusive;

use rand::distr::{Distribution, StandardUniform};
use rand::{Rng, RngExt};
use silizium::zigbee::security::man::Key;

use crate::ember::{Eui64, PanId};

const EUI64_LENGTH: usize = 8;
const LOCAL_BIT: u8 = 0x02;
const MULTICAST_BIT: u8 = 0x01;
const VALID_PAN_IDS: RangeInclusive<u16> = 0x0000..=0xFFFE;

/// Identity and network-key material used to form a Zigbee network.
///
/// Credentials contain the extended PAN ID, PAN ID, trust-center EUI-64, and
/// network key. Combine them with a preconfigured link key and the desired
/// radio settings through [`InitializationParameters::new`](crate::InitializationParameters::new),
/// then select [`Startup::Initialize`](crate::Startup::Initialize).
///
/// This type contains secret key material. Avoid logging its [`Debug`] output,
/// and store serialized or copied values using protections appropriate for
/// network credentials.
///
/// Random credentials can be sampled with [`RngExt::random`](rand::RngExt::random).
/// The sampling implementation accepts any random-number generator, so
/// production callers must supply a cryptographically secure generator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NetworkCredentials {
    pub(crate) extended_pan_id: Eui64,
    pub(crate) pan_id: PanId,
    pub(crate) trust_center_eui64: Eui64,
    pub(crate) network_key: Key,
}

impl NetworkCredentials {
    /// Creates credentials from explicit network identifiers and key material.
    ///
    /// `extended_pan_id` identifies the Zigbee network, `pan_id` is its
    /// 16-bit identifier, `trust_center_eui64` identifies its trust center, and
    /// `network_key` secures network-layer traffic.
    #[must_use]
    pub const fn new(
        extended_pan_id: Eui64,
        pan_id: PanId,
        trust_center_eui64: Eui64,
        network_key: Key,
    ) -> Self {
        Self {
            extended_pan_id,
            pan_id,
            trust_center_eui64,
            network_key,
        }
    }
}

impl Distribution<NetworkCredentials> for StandardUniform {
    /// Samples random network identifiers and a random network key.
    ///
    /// Both generated EUI-64 values are locally administered unicast
    /// addresses and exclude the all-zero and all-one sentinel values. The PAN
    /// ID excludes the reserved broadcast value `0xFFFF`.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> NetworkCredentials {
        NetworkCredentials {
            extended_pan_id: random_mac_addr8(rng),
            pan_id: rng.random_range(VALID_PAN_IDS),
            trust_center_eui64: random_mac_addr8(rng),
            network_key: rng.random(),
        }
    }
}

fn random_mac_addr8<T>(rng: &mut T) -> Eui64
where
    T: Rng + ?Sized,
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
