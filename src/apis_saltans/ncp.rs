//! `apis_saltans_hw::Driver` implementation for [`Ncp`].
//!
//! This module maps the generic hardware-driver operations expected by
//! `apis-saltans` to EZSP command traits. Direct NCP operations are forwarded to
//! the wrapped transport, while APS sends use [`Ncp`] so that EZSP
//! `messageSent` callbacks can be correlated with the originating request. The
//! APS profile and cluster are taken from `apis_saltans_hw::Datagram` metadata;
//! the local source endpoint is selected by [`Ncp`] from the registered
//! endpoint output clusters.

use std::collections::BTreeMap;
use std::time::Duration;

use apis_saltans_hw::core::{Application, Destination, IeeeAddress};
use apis_saltans_hw::{Clusters, Datagram, Driver, Error, FoundNetwork, ScannedChannel};

use crate::ember::concentrator;
use crate::{
    Configuration, Messaging, MulticastOptions, Ncp, Networking, Security, Transport, Utilities,
};

mod builder;
mod event_handler;

const DEFAULT_BROADCAST_RADIUS: u8 = 0;
const DEFAULT_MULTICAST_HOPS: u8 = 0;
const DEFAULT_MULTICAST_NONMEMBER_RADIUS: u8 = 0;

impl<T> Driver for Ncp<T>
where
    T: Configuration + Security + Messaging + Networking + Utilities + Transport,
{
    async fn get_endpoints(&self) -> Result<BTreeMap<Application, Clusters>, Error> {
        Ok(self
            .endpoints
            .iter()
            .enumerate()
            .map_while(|(index, cluster)| index.checked_add(1).map(|index| (index, cluster)))
            .map_while(|(index, cluster)| u8::try_from(index).ok().map(|index| (index, cluster)))
            .filter_map(|(index, cluster)| Application::new(index).map(|app| (app, cluster.into())))
            .collect())
    }

    async fn get_pan_id(&mut self) -> Result<u16, Error> {
        Ok(self.transport.lock().await.get_node_id().await?)
    }

    async fn get_ieee_address(&mut self) -> Result<IeeeAddress, Error> {
        Ok(self.transport.lock().await.get_eui64().await?.into())
    }

    async fn scan_networks(
        &mut self,
        channel_mask: u32,
        duration: u8,
    ) -> Result<Vec<FoundNetwork>, Error> {
        Self::scan_networks(self, channel_mask, duration)
            .await
            .map(|results| results.into_iter().map(Into::into).collect())
            .map_err(Into::into)
    }

    async fn scan_channels(
        &mut self,
        channel_mask: u32,
        duration: u8,
    ) -> Result<Vec<ScannedChannel>, Error> {
        Self::scan_channels(self, channel_mask, duration)
            .await
            .map(|results| results.into_iter().map(Into::into).collect())
            .map_err(Into::into)
    }

    async fn allow_joins(&mut self, duration: Duration) -> Result<Duration, Error> {
        let seconds = u8::try_from(duration.as_secs()).unwrap_or(u8::MAX);
        self.transport
            .lock()
            .await
            .permit_joining(seconds.into())
            .await?;
        Ok(Duration::from_secs(u64::from(seconds)))
    }

    async fn route_request(&mut self, radius: u8) -> Result<(), Error> {
        Ok(self
            .transport
            .lock()
            .await
            .send_many_to_one_route_request(concentrator::Type::HighRam, radius)
            .await?)
    }

    async fn short_id_to_ieee_address(&mut self, short_id: u16) -> Result<IeeeAddress, Error> {
        Ok(self
            .transport
            .lock()
            .await
            .lookup_eui64_by_node_id(short_id)
            .await?
            .into())
    }

    async fn ieee_address_to_short_id(&mut self, ieee_address: IeeeAddress) -> Result<u16, Error> {
        Ok(self
            .transport
            .lock()
            .await
            .lookup_node_id_by_eui64(ieee_address.into())
            .await?)
    }

    async fn transmit(
        &mut self,
        destination: Destination,
        datagram: Datagram,
    ) -> Result<(), Error> {
        let (metadata, payload) = datagram.into_parts();
        let profile = metadata.profile();
        let profile_id = profile.into();
        let cluster_id = metadata.cluster_id();

        match destination {
            Destination::Device(device) => {
                self.unicast(
                    device.device().into(),
                    profile_id,
                    cluster_id,
                    device.endpoint().into(),
                    payload,
                )
                .await
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
                .await
            }
            Destination::Group(group_id) => {
                self.multicast(
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
                .await
            }
        }
        .map_err(Into::into)
    }
}
