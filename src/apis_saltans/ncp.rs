//! `apis_saltans_hw::Driver` adapter for [`Ncp`].
//!
//! [`ZigbeeNcp`] wraps the plain host-side [`Ncp`] and registers every supplied
//! endpoint as part of construction. Consequently, a value that implements
//! `apis_saltans_hw::Driver` always has the same endpoints configured on the
//! device and available through `Driver::get_endpoints`.
//! Endpoint registration is implemented by the private [`AddSimpleDescriptors`]
//! extension trait, which translates the `apis-saltans` descriptors into
//! sequential [`Ncp::add_endpoint`] calls.
//!
//! The adapter maps the generic hardware-driver operations expected by
//! `apis-saltans` to EZSP command traits. Direct NCP operations are forwarded to
//! the wrapped transport, while APS sends use the wrapped [`Ncp`] so that EZSP
//! `messageSent` callbacks can be correlated with the originating request. The
//! APS profile and cluster are taken from `apis_saltans_hw::Datagram` metadata;
//! the local source endpoint is selected by [`Ncp`] from the registered
//! endpoint output clusters.

use std::time::Duration;

use apis_saltans_hw::core::{Destination, IeeeAddress, Profile};
use apis_saltans_hw::zdp::{AppFlags, SimpleDescriptor};
use apis_saltans_hw::{Datagram, Driver, Error, FoundNetwork, HwResponse, ScannedChannel};
use log::error;

use crate::ember::concentrator;
use crate::{Configuration, Endpoint, Messaging, MulticastOptions, Ncp, Networking, Utilities};

const DEFAULT_BROADCAST_RADIUS: u8 = 0;
const DEFAULT_MULTICAST_HOPS: u8 = 0;
const DEFAULT_MULTICAST_NONMEMBER_RADIUS: u8 = 0;

impl<T> Driver for Ncp<T>
where
    T: Configuration + Messaging + Networking + Utilities + Send + Sync,
{
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
        Ok(self.transport.get_node_id().await?)
    }

    async fn get_ieee_address(&mut self) -> Result<IeeeAddress, Error> {
        Ok(self.transport.get_eui64().await?.into())
    }

    async fn scan_networks(
        &mut self,
        channel_mask: u32,
        duration: u8,
    ) -> Result<Vec<FoundNetwork>, Error> {
        self.scan_networks(channel_mask, duration)
            .await
            .map(|results| results.into_iter().map(Into::into).collect())
            .map_err(Into::into)
    }

    async fn scan_channels(
        &mut self,
        channel_mask: u32,
        duration: u8,
    ) -> Result<Vec<ScannedChannel>, Error> {
        self.scan_channels(channel_mask, duration)
            .await
            .map(|results| results.into_iter().map(Into::into).collect())
            .map_err(Into::into)
    }

    async fn allow_joins(&mut self, duration: Duration) -> Result<Duration, Error> {
        let seconds = u8::try_from(duration.as_secs()).unwrap_or(u8::MAX);
        self.transport.permit_joining(seconds.into()).await?;
        Ok(Duration::from_secs(u64::from(seconds)))
    }

    async fn route_request(&mut self, radius: u8) -> Result<(), Error> {
        Ok(self
            .transport
            .send_many_to_one_route_request(concentrator::Type::HighRam, radius)
            .await?)
    }

    async fn short_id_to_ieee_address(&mut self, short_id: u16) -> Result<IeeeAddress, Error> {
        Ok(self
            .transport
            .lookup_eui64_by_node_id(short_id)
            .await?
            .into())
    }

    async fn ieee_address_to_short_id(&mut self, ieee_address: IeeeAddress) -> Result<u16, Error> {
        Ok(self
            .transport
            .lookup_node_id_by_eui64(ieee_address.into())
            .await?)
    }

    async fn transmit(
        &mut self,
        destination: Destination,
        datagram: Datagram,
    ) -> Result<HwResponse, Error> {
        let (metadata, payload) = datagram.into_parts();
        let profile = metadata.profile();
        let profile_id = profile.into();
        let cluster_id = metadata.cluster_id();

        let stack_response = match destination {
            Destination::Device(device) => {
                self.unicast(
                    device.device().into(),
                    profile_id,
                    cluster_id,
                    device.endpoint().into(),
                    payload,
                )
                .await?
            }
            Destination::Broadcast(broadcast) => {
                self.broadcast(
                    broadcast.address().as_u16(),
                    DEFAULT_BROADCAST_RADIUS,
                    profile_id,
                    cluster_id,
                    broadcast.endpoint().into(),
                    payload,
                )
                .await?
            }
            Destination::Group(group_id) => {
                let (stack_response, _seq) = self
                    .multicast(
                        group_id.as_u16(),
                        MulticastOptions::new(
                            DEFAULT_MULTICAST_HOPS,
                            DEFAULT_MULTICAST_NONMEMBER_RADIUS,
                        ),
                        profile_id,
                        cluster_id,
                        profile.broadcast_endpoint().into(),
                        payload,
                    )
                    .await?;
                stack_response
            }
        };

        Ok(HwResponse::new(stack_response))
    }
}

impl TryFrom<Endpoint> for SimpleDescriptor {
    type Error = Endpoint;

    fn try_from(value: Endpoint) -> Result<Self, Self::Error> {
        let Ok(profile) = Profile::try_from(value.profile_id) else {
            return Err(value);
        };

        let Ok(endpoint) = apis_saltans_hw::core::Endpoint::try_from(value.id) else {
            return Err(value);
        };

        Ok(SimpleDescriptor::new(
            endpoint,
            profile,
            value.device_id,
            AppFlags::from_bits_retain(value.app_flags),
            value.input_clusters,
            value.output_clusters,
        ))
    }
}
