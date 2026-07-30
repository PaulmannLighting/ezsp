//! `apis_saltans_hw::Driver` implementation for [`Ncp`].
//!
//! The implementation is attached directly to the high-level [`Ncp`]; there is
//! no feature-specific wrapper. The `Ncp` already owns the endpoint descriptors
//! registered by [`crate::Builder`], so `Driver::get_endpoints` converts that
//! stored list back to `apis-saltans` simple descriptors. Unsupported profile
//! IDs are logged and omitted from the result.
//!
//! Driver operations map to EZSP as follows:
//!
//! - PAN and IEEE identity use `getNetworkParameters` and `getEui64`,
//!   respectively;
//! - active and energy scans translate the typed channel mask and duration into
//!   EZSP values and use [`Ncp`] callback aggregation;
//! - permit-joining duration is truncated to whole seconds and clamped to
//!   `u8::MAX` seconds;
//! - route requests use a high-RAM many-to-one concentrator request;
//! - address translation uses typed device short IDs with the EZSP
//!   address-table lookup commands; and
//! - APSDE request transmission delegates to explicit-source
//!   [`Ncp::unicast`], [`Ncp::broadcast`], or [`Ncp::multicast`].
//!
//! The APS profile, cluster, source endpoint, correlation counter, radius, and
//! transmission options come from the supplied APSDE request. EZSP assigns its
//! own APS sequence internally. Acknowledged transmission, APS security, and
//! fragmentation-permitted flags control EZSP APS retry, encryption, and host
//! fragmentation, respectively; the mapped options are combined with the
//! baseline options stored by [`Ncp`].
//! Network and broadcast destinations preserve their requested endpoint, while
//! groups use the profile's broadcast endpoint and zero nonmember radius.
//!
//! A successful transmit call means that EZSP accepted the request. Its later
//! `messageSent` callback is translated into an APSDE data confirmation.

use std::time::Duration;

use apis_saltans_hw::aps::apsde::{Alias, DataRequest, NetworkAddress, RequestDestination};
use apis_saltans_hw::core::short_id::Device;
use apis_saltans_hw::core::{IeeeAddress, Profile};
use apis_saltans_hw::zdp::SimpleDescriptor;
use apis_saltans_hw::{
    ChannelMask, Driver, Error, FoundNetwork, ScanDuration, ScannedChannel, TxOptions,
};
use bytes::Bytes;
use log::error;

use crate::ember::concentrator;
use crate::{Messaging, MulticastOptions, Ncp, Networking, Utilities};

const DEFAULT_RADIUS_COUNTER: u8 = 0;
const DEFAULT_MULTICAST_NONMEMBER_RADIUS: u8 = 0;
const GROUP_BROADCAST_ADDRESS: u16 = 0xFFFF;
const SUPPORTED_TX_OPTIONS: TxOptions = TxOptions::ACKNOWLEDGED_TRANSMISSION
    .union(TxOptions::SECURITY_ENABLED)
    .union(TxOptions::FRAGMENTATION_PERMITTED);

/// Invalid value encountered while crossing the hardware driver boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
enum BoundaryError {
    /// The NCP reported a channel outside the Zigbee page-zero range.
    #[error("NCP returned invalid Zigbee channel: {0}")]
    InvalidChannel(u8),

    /// The NCP reported a short ID that is not allocated to a device.
    #[error("NCP returned invalid device short ID: {0:#06X}")]
    InvalidShortId(u16),

    /// Address lookup produced a reserved or broadcast NWK address.
    #[error("NCP returned invalid network address: {0:#06X}")]
    InvalidNetworkAddress(u16),

    /// A group transmission used a profile unknown to `apis-saltans`.
    #[error("Cannot transmit APS group frame with unknown profile: {0:#06X}")]
    UnknownProfile(u16),

    /// EZSP cannot select bindings from an APSDE bound request.
    #[error("EZSP cannot transmit an APSDE request using binding resolution")]
    BoundDestination,

    /// EZSP cannot preserve an APSDE NWK source alias in this send path.
    #[error("EZSP cannot transmit this APSDE request with a NWK source alias")]
    SourceAlias,

    /// EZSP multicast supports only the all-devices NWK broadcast selector.
    #[error("Unsupported APSDE group broadcast address: {0:#06X}")]
    GroupBroadcastAddress(u16),

    /// EZSP cannot set a hop radius on an APS unicast request.
    #[error("Unsupported APSDE unicast radius: {0}")]
    UnicastRadius(u8),

    /// The request uses APSDE transmission options unavailable through EZSP.
    #[error("Unsupported APSDE transmission options: {0}")]
    TxOptions(TxOptions),
}

impl Driver for Ncp {
    async fn get_endpoints(&self) -> Result<Box<[SimpleDescriptor]>, Error> {
        Ok(self
            .endpoints
            .iter()
            .cloned()
            .filter_map(|endpoint| {
                endpoint
                    .try_into()
                    .inspect_err(|error| error!("Failed to translate endpoint: {error:?}"))
                    .ok()
            })
            .collect())
    }

    async fn get_pan_id(&mut self) -> Result<u16, Error> {
        Ok(self.connection.get_network_parameters().await?.1.pan_id())
    }

    async fn get_ieee_address(&mut self) -> Result<IeeeAddress, Error> {
        Ok(self.connection.get_eui64().await?.into())
    }

    async fn scan_networks(
        &mut self,
        channel_mask: ChannelMask,
        duration: ScanDuration,
    ) -> Result<Vec<FoundNetwork>, Error> {
        self.scan_networks(channel_mask.bits(), duration.into())
            .await
            .map_err(Error::backend)?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()
            .map_err(|channel| Error::backend(BoundaryError::InvalidChannel(channel)))
    }

    async fn scan_channels(
        &mut self,
        channel_mask: ChannelMask,
        duration: ScanDuration,
    ) -> Result<Vec<ScannedChannel>, Error> {
        self.scan_channels(channel_mask.bits(), duration.into())
            .await
            .map_err(Error::backend)?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()
            .map_err(|channel| Error::backend(BoundaryError::InvalidChannel(channel)))
    }

    async fn allow_joins(&mut self, duration: Duration) -> Result<Duration, Error> {
        let seconds = u8::try_from(duration.as_secs()).unwrap_or(u8::MAX);
        self.connection.permit_joining(seconds.into()).await?;
        Ok(Duration::from_secs(u64::from(seconds)))
    }

    async fn route_request(&mut self, radius: u8) -> Result<(), Error> {
        Ok(self
            .connection
            .send_many_to_one_route_request(concentrator::Type::HighRam, radius)
            .await?)
    }

    async fn short_id_to_ieee_address(&mut self, short_id: Device) -> Result<IeeeAddress, Error> {
        Ok(self
            .connection
            .lookup_eui64_by_node_id(short_id.into())
            .await?
            .into())
    }

    async fn ieee_address_to_short_id(
        &mut self,
        ieee_address: IeeeAddress,
    ) -> Result<Device, Error> {
        let short_id = self
            .connection
            .lookup_node_id_by_eui64(ieee_address.into())
            .await?;

        Device::new(short_id).ok_or_else(|| Error::backend(BoundaryError::InvalidShortId(short_id)))
    }

    async fn transmit(&mut self, request: DataRequest<Bytes>, counter: u8) -> Result<(), Error> {
        let destination = request.destination();
        let profile_id = request.profile_id();
        let cluster_id = request.cluster_id();
        let source_endpoint = request.source_endpoint().get().into();
        let radius = request.radius_counter();
        let tx_options = request.tx_options();
        let alias = request.alias();
        let fragmentation_permitted = validate_tx_options(tx_options)?;
        reject_alias(alias)?;
        let aps_options = tx_options.into();
        let payload = request.into_asdu();

        let result = match destination {
            RequestDestination::Bound => {
                return Err(Error::backend(BoundaryError::BoundDestination));
            }
            RequestDestination::Network { address, endpoint } => {
                reject_unicast_radius(radius)?;
                self.unicast(
                    source_endpoint,
                    address.as_u16(),
                    profile_id,
                    cluster_id,
                    endpoint.into(),
                    payload,
                    aps_options,
                    counter,
                    fragmentation_permitted,
                )
                .await
            }
            RequestDestination::Extended { address, endpoint } => {
                reject_unicast_radius(radius)?;
                let short_id = self
                    .connection
                    .lookup_node_id_by_eui64(address.into())
                    .await
                    .map_err(Error::backend)?;
                let short_id = NetworkAddress::new(short_id).ok_or_else(|| {
                    Error::backend(BoundaryError::InvalidNetworkAddress(short_id))
                })?;
                self.unicast(
                    source_endpoint,
                    short_id.as_u16(),
                    profile_id,
                    cluster_id,
                    endpoint.into(),
                    payload,
                    aps_options,
                    counter,
                    fragmentation_permitted,
                )
                .await
            }
            RequestDestination::Broadcast { address, endpoint } => {
                self.broadcast(
                    source_endpoint,
                    address.as_u16(),
                    profile_id,
                    cluster_id,
                    endpoint.into(),
                    payload,
                    radius,
                    aps_options,
                    counter,
                )
                .await
            }
            RequestDestination::Group {
                address,
                broadcast_address,
            } => {
                if broadcast_address.as_u16() != GROUP_BROADCAST_ADDRESS {
                    return Err(Error::backend(BoundaryError::GroupBroadcastAddress(
                        broadcast_address.as_u16(),
                    )));
                }
                let Ok(profile) = Profile::try_from(profile_id) else {
                    return Err(Error::backend(BoundaryError::UnknownProfile(profile_id)));
                };

                self.multicast(
                    source_endpoint,
                    address.as_u16(),
                    profile_id,
                    cluster_id,
                    profile.broadcast_endpoint().into(),
                    payload,
                    MulticastOptions::new(radius, DEFAULT_MULTICAST_NONMEMBER_RADIUS),
                    aps_options,
                    counter,
                )
                .await
            }
        };

        result.map_err(Error::backend)
    }
}

fn validate_tx_options(tx_options: TxOptions) -> Result<bool, Error> {
    let unsupported = tx_options.difference(SUPPORTED_TX_OPTIONS);
    if unsupported.is_empty() {
        Ok(tx_options.contains(TxOptions::FRAGMENTATION_PERMITTED))
    } else {
        Err(Error::backend(BoundaryError::TxOptions(unsupported)))
    }
}

fn reject_alias(alias: Alias) -> Result<(), Error> {
    if matches!(alias, Alias::None) {
        Ok(())
    } else {
        Err(Error::backend(BoundaryError::SourceAlias))
    }
}

fn reject_unicast_radius(radius: u8) -> Result<(), Error> {
    if radius == DEFAULT_RADIUS_COUNTER {
        Ok(())
    } else {
        Err(Error::backend(BoundaryError::UnicastRadius(radius)))
    }
}

#[cfg(test)]
mod tests {
    use apis_saltans_hw::TxOptions;

    use super::{DEFAULT_RADIUS_COUNTER, reject_unicast_radius, validate_tx_options};

    const NON_DEFAULT_RADIUS: u8 = DEFAULT_RADIUS_COUNTER + 1;

    #[test]
    fn validates_supported_transmission_options() {
        assert!(
            !validate_tx_options(TxOptions::ACKNOWLEDGED_TRANSMISSION)
                .expect("acknowledgement is supported")
        );
        assert!(
            validate_tx_options(TxOptions::FRAGMENTATION_PERMITTED)
                .expect("fragmentation is supported")
        );
    }

    #[test]
    fn rejects_unsupported_transmission_options() {
        assert!(validate_tx_options(TxOptions::USE_NWK_KEY).is_err());
        assert!(validate_tx_options(TxOptions::INCLUDE_EXTENDED_NONCE).is_err());
    }

    #[test]
    fn accepts_only_the_default_unicast_radius() {
        assert!(reject_unicast_radius(DEFAULT_RADIUS_COUNTER).is_ok());
        assert!(reject_unicast_radius(NON_DEFAULT_RADIUS).is_err());
    }
}
