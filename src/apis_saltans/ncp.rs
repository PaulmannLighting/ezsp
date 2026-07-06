//! `apis_saltans_hw::NcpDriver` implementation for [`crate::Ncp`].
//!
//! This module maps the generic hardware-driver operations expected by
//! `apis-saltans` to EZSP command traits. Direct NCP operations are forwarded to
//! the wrapped transport, while APS sends use [`crate::Ncp`] so that EZSP
//! `messageSent` callbacks can be correlated with the originating request.

use std::collections::BTreeMap;
use std::time::Duration;

use apis_saltans_core::Endpoint;
use apis_saltans_hw::{
    Error, FoundNetwork, Frame, NcpDriver, ParallelUnicastResult, ScannedChannel,
};
use macaddr::MacAddr8;

use crate::ember::concentrator;
use crate::{Configuration, Messaging, Ncp, Networking, Security, Utilities};

mod builder;
mod event_handler;

impl<T> NcpDriver for Ncp<T>
where
    T: Configuration + Security + Messaging + Networking + Utilities + Send + Sync,
{
    fn next_transaction_seq(&mut self) -> u8 {
        Self::next_transaction_seq(self)
    }

    async fn get_pan_id(&mut self) -> Result<u16, Error> {
        Ok(self.get_node_id().await?)
    }

    async fn get_ieee_address(&mut self) -> Result<MacAddr8, Error> {
        Ok(self.get_eui64().await?)
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

    async fn get_neighbors(&mut self) -> Result<BTreeMap<MacAddr8, u16>, Error> {
        Ok(Self::get_neighbors(self).await?)
    }

    async fn route_request(&mut self, radius: u8) -> Result<(), Error> {
        Ok(self
            .send_many_to_one_route_request(concentrator::Type::HighRam, radius)
            .await?)
    }

    async fn short_id_to_ieee_address(&mut self, short_id: u16) -> Result<MacAddr8, Error> {
        Ok(self.lookup_eui64_by_node_id(short_id).await?)
    }

    async fn ieee_address_to_short_id(&mut self, ieee_address: MacAddr8) -> Result<u16, Error> {
        Ok(self.lookup_node_id_by_eui64(ieee_address).await?)
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
            metadata.profile().map(Into::into),
            metadata.cluster_id(),
            metadata
                .source_endpoint()
                .unwrap_or(destination_endpoint)
                .into(),
            destination_endpoint.into(),
            payload.iter().copied().collect(),
        )
        .await?
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
            metadata.profile().map(Into::into),
            metadata.cluster_id(),
            metadata.source_endpoint().unwrap_or_default().into(),
            payload.iter().copied().collect(),
        )
        .await?
        .await
        .map_err(Into::into)
    }

    async fn broadcast(&mut self, short_id: u16, radius: u8, frame: Frame) -> Result<u8, Error> {
        let (metadata, payload) = frame.into_parts();
        self.broadcast(
            short_id,
            radius,
            metadata.profile().map(Into::into),
            metadata.cluster_id(),
            metadata.source_endpoint().unwrap_or_default().into(),
            payload.iter().copied().collect(),
        )
        .await
        .map_err(Into::into)
    }

    async fn parallel_unicast(
        &mut self,
        targets: BTreeMap<u16, Box<[Endpoint]>>,
        frame: Frame,
    ) -> ParallelUnicastResult {
        let (metadata, payload) = frame.into_parts();
        let mut responses = BTreeMap::new();

        for (short_id, endpoint) in targets.into_iter().flat_map(|(short_id, endpoints)| {
            endpoints
                .into_iter()
                .map(move |endpoint| (short_id, endpoint))
        }) {
            responses.insert(
                (short_id, endpoint),
                Self::unicast(
                    self,
                    short_id,
                    metadata.profile().map(Into::into),
                    metadata.cluster_id(),
                    metadata.source_endpoint().unwrap_or(endpoint).into(),
                    endpoint.into(),
                    payload.iter().copied().collect(),
                )
                .await?,
            );
        }

        let mut results = BTreeMap::new();

        for (ident, response) in responses {
            results.insert(ident, response.await.map_err(Into::into));
        }

        Ok(results)
    }
}
