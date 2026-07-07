//! `apis_saltans_hw::NcpDriver` implementation for [`crate::Ncp`].
//!
//! This module maps the generic hardware-driver operations expected by
//! `apis-saltans` to EZSP command traits. Direct NCP operations are forwarded to
//! the wrapped transport, while APS sends use [`crate::Ncp`] so that EZSP
//! `messageSent` callbacks can be correlated with the originating request. The
//! APS profile and cluster are taken from `apis_saltans_hw::Frame` metadata;
//! the local source endpoint is selected by [`crate::Ncp`] from the registered
//! endpoint output clusters. Unicast sends address one destination endpoint per
//! call; callers that need fan-out should issue multiple unicast requests.

use std::collections::BTreeMap;
use std::time::Duration;

use apis_saltans_core::{Endpoint, IeeeAddress};
use apis_saltans_hw::{Error, FoundNetwork, Frame, NcpDriver, ScannedChannel};

use crate::ember::concentrator;
use crate::{Configuration, Messaging, Ncp, Networking, Security, Utilities};

mod builder;
mod event_handler;

impl<T> NcpDriver for Ncp<T>
where
    T: Configuration + Security + Messaging + Networking + Utilities + Send + Sync,
{
    async fn get_pan_id(&mut self) -> Result<u16, Error> {
        Ok(self.get_node_id().await?)
    }

    async fn get_ieee_address(&mut self) -> Result<IeeeAddress, Error> {
        Ok(self.get_eui64().await?.into())
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
        self.permit_joining(seconds.into()).await?;
        Ok(Duration::from_secs(u64::from(seconds)))
    }

    async fn get_neighbors(&mut self) -> Result<BTreeMap<IeeeAddress, u16>, Error> {
        Ok(Self::get_neighbors(self)
            .await?
            .into_iter()
            .map(|(ieee_address, short_id)| (ieee_address.into(), short_id))
            .collect())
    }

    async fn route_request(&mut self, radius: u8) -> Result<(), Error> {
        Ok(self
            .send_many_to_one_route_request(concentrator::Type::HighRam, radius)
            .await?)
    }

    async fn short_id_to_ieee_address(&mut self, short_id: u16) -> Result<IeeeAddress, Error> {
        Ok(self.lookup_eui64_by_node_id(short_id).await?.into())
    }

    async fn ieee_address_to_short_id(&mut self, ieee_address: IeeeAddress) -> Result<u16, Error> {
        Ok(self.lookup_node_id_by_eui64(ieee_address.into()).await?)
    }

    async fn unicast(
        &mut self,
        short_id: u16,
        destination_endpoint: Endpoint,
        frame: Frame,
    ) -> Result<u8, Error> {
        let (metadata, payload) = frame.into_parts();
        self.unicast(
            short_id,
            metadata.profile().into(),
            metadata.cluster_id(),
            destination_endpoint.into(),
            payload.into_iter().collect(),
        )
        .await
        .map_err(Into::into)
    }

    async fn multicast(
        &mut self,
        group_id: u16,
        hops: u8,
        radius: u8,
        frame: Frame,
    ) -> Result<u8, Error> {
        let (metadata, payload) = frame.into_parts();
        self.multicast(
            group_id,
            hops,
            radius,
            metadata.profile().into(),
            metadata.cluster_id(),
            metadata.profile().broadcast_endpoint().into(),
            payload.into_iter().collect(),
        )
        .await
        .map_err(Into::into)
    }

    async fn broadcast(&mut self, short_id: u16, radius: u8, frame: Frame) -> Result<u8, Error> {
        let (metadata, payload) = frame.into_parts();
        self.broadcast(
            short_id,
            radius,
            metadata.profile().into(),
            metadata.cluster_id(),
            metadata.profile().broadcast_endpoint().into(),
            payload.into_iter().collect(),
        )
        .await
        .map_err(Into::into)
    }
}
