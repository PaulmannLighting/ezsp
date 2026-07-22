//! `apis_saltans_hw::Driver` implementation for [`Ncp`].
//!
//! The implementation is attached directly to the high-level [`Ncp`]; there is
//! no feature-specific wrapper. The `Ncp` already owns the endpoint descriptors
//! registered by [`crate::Builder`], so `Driver::get_endpoints` converts that
//! stored list back to `apis-saltans` simple descriptors. Unsupported profile
//! IDs and reserved endpoint numbers are logged and omitted from the result.
//!
//! Driver operations map to EZSP as follows:
//!
//! - network and IEEE identity use `getNodeId` and `getEui64`, respectively;
//! - active and energy scans use [`Ncp`] callback aggregation;
//! - permit-joining duration is truncated to whole seconds and clamped to
//!   `u8::MAX` seconds;
//! - route requests use a high-RAM many-to-one concentrator request;
//! - address translation uses the EZSP address-table lookup commands; and
//! - datagram transmission delegates to [`Ncp::unicast`], [`Ncp::broadcast`],
//!   or [`Ncp::multicast`].
//!
//! The APS profile and cluster come from `apis_saltans_hw::Datagram` metadata.
//! Device destinations preserve their requested endpoint, broadcasts use their
//! requested endpoint with radius zero, and groups use the profile's broadcast
//! endpoint with zero hops and nonmember radius. [`Ncp`] selects the local
//! source endpoint from its registered output clusters.
//!
//! A transmit call returns `apis_saltans_hw::HwResponse` after the EZSP send
//! transaction is accepted. That response owns the deferred [`crate::StackResponse`]
//! and reports the later `messageSent` status when awaited.

use std::time::Duration;

use apis_saltans_hw::core::{Destination, IeeeAddress};
use apis_saltans_hw::zdp::SimpleDescriptor;
use apis_saltans_hw::{Datagram, Driver, Error, FoundNetwork, HwResponse, ScannedChannel};
use log::error;

use crate::ember::concentrator;
use crate::{Messaging, MulticastOptions, Ncp, Networking, Utilities};

const DEFAULT_BROADCAST_RADIUS: u8 = 0;
const DEFAULT_MULTICAST_HOPS: u8 = 0;
const DEFAULT_MULTICAST_NONMEMBER_RADIUS: u8 = 0;

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
        Ok(self.connection.get_node_id().await?)
    }

    async fn get_ieee_address(&mut self) -> Result<IeeeAddress, Error> {
        Ok(self.connection.get_eui64().await?.into())
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
        self.connection.permit_joining(seconds.into()).await?;
        Ok(Duration::from_secs(u64::from(seconds)))
    }

    async fn route_request(&mut self, radius: u8) -> Result<(), Error> {
        Ok(self
            .connection
            .send_many_to_one_route_request(concentrator::Type::HighRam, radius)
            .await?)
    }

    async fn short_id_to_ieee_address(&mut self, short_id: u16) -> Result<IeeeAddress, Error> {
        Ok(self
            .connection
            .lookup_eui64_by_node_id(short_id)
            .await?
            .into())
    }

    async fn ieee_address_to_short_id(&mut self, ieee_address: IeeeAddress) -> Result<u16, Error> {
        Ok(self
            .connection
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
