//! Network manager for Zigbee networks.

use std::collections::BTreeMap;
use std::io;
use std::time::Duration;

use log::{debug, info};
use macaddr::MacAddr8;
use tokio::sync::mpsc::Receiver;
use zigbee::{Endpoint, Profile};
use zigbee_nwk::{ApsMetadata, Frame, NetworkManager};

use self::builder::Builder;
use self::collect_networks_found::CollectNetworksFound;
use self::message_handler::Handlers;
use crate::ember::message::Destination;
use crate::ember::{aps, concentrator};
use crate::error::Status;
use crate::ezsp::network::scan;
use crate::types::ByteSizedVec;
use crate::{Callback, Configuration, Error, Messaging, Networking, Security, Utilities, ember};

mod builder;
mod collect_networks_found;
mod message_handler;

/// Network manager for Zigbee networks.
pub struct EzspNetworkManager<T> {
    transport: T,
    profile: Profile,
    aps_options: aps::Options,
    handlers: Handlers,
    message_tag: u8,
    aps_seq: u8,
    transaction_seq: u8,
}

impl<T> EzspNetworkManager<T> {
    /// Creates a new `NetworkManager` with the given transport.
    #[must_use]
    pub(crate) const fn new(
        transport: T,
        profile: Profile,
        aps_options: aps::Options,
        handlers: Handlers,
    ) -> Self {
        Self {
            transport,
            profile,
            aps_options,
            handlers,
            message_tag: 0,
            aps_seq: 0,
            transaction_seq: 0,
        }
    }

    /// Creates a new `Builder` for constructing a `NetworkManager`.
    #[must_use]
    pub fn build(transport: T, callbacks_rx: Receiver<Callback>) -> Builder<T> {
        Builder::new(transport, callbacks_rx)
    }

    /// Returns the next message tag and increments the internal counter.
    const fn next_message_tag(&mut self) -> u8 {
        let tag = self.message_tag;
        self.message_tag = self.message_tag.wrapping_add(1);
        tag
    }

    /// Returns the next APS sequence number and increments the internal counter.
    const fn next_aps_seq(&mut self) -> u8 {
        let seq = self.aps_seq;
        self.aps_seq = self.aps_seq.wrapping_add(1);
        seq
    }

    /// Registers a new channel for receiving callbacks.
    #[expect(clippy::needless_pass_by_ref_mut)]
    async fn register_handler(&mut self, size: usize) -> Receiver<Callback> {
        let (tx, rx) = tokio::sync::mpsc::channel(size);
        self.handlers.lock().await.push(tx);
        rx
    }

    /// Creates a new APS frame with the given parameters.
    fn next_aps_frame(
        &mut self,
        aps_metadata: ApsMetadata,
        destination_endpoint: Endpoint,
        group_id: u16,
    ) -> aps::Frame {
        aps::Frame::new(
            aps_metadata.profile().unwrap_or(self.profile).into(),
            aps_metadata.cluster_id(),
            aps_metadata.source_endpoint().map_or(0x01, u8::from),
            destination_endpoint.into(),
            self.aps_options,
            group_id,
            self.next_aps_seq(),
        )
    }
}

impl<T> NetworkManager for EzspNetworkManager<T>
where
    T: Configuration + Security + Messaging + Networking + Utilities,
{
    fn next_transaction_seq(&mut self) -> u8 {
        let seq = self.transaction_seq;
        self.transaction_seq = self.transaction_seq.wrapping_add(1);
        seq
    }

    async fn get_pan_id(&mut self) -> Result<u16, zigbee_nwk::Error> {
        let (_, parameters) = self.transport.get_network_parameters().await?;
        Ok(parameters.pan_id())
    }

    async fn scan_networks(
        &mut self,
        channel_mask: u32,
        duration: u8,
    ) -> Result<Vec<zigbee_nwk::FoundNetwork>, zigbee_nwk::Error> {
        let mut rx = self.register_handler(16).await;
        self.transport
            .start_scan(scan::Type::ActiveScan, channel_mask, duration)
            .await?;
        Ok(rx.collect_networks_found().await?)
    }

    async fn scan_channels(
        &mut self,
        channel_mask: u32,
        duration: u8,
    ) -> Result<Vec<zigbee_nwk::ScannedChannel>, zigbee_nwk::Error> {
        let mut rx = self.register_handler(16).await;
        self.transport
            .start_scan(scan::Type::EnergyScan, channel_mask, duration)
            .await?;
        Ok(rx.collect_scanned_channels().await?)
    }

    async fn allow_joins(&mut self, duration: Duration) -> Result<(), zigbee_nwk::Error> {
        info!("Allowing joins for {} seconds.", duration.as_secs());
        self.transport
            .permit_joining(u8::try_from(duration.as_secs()).unwrap_or(u8::MAX).into())
            .await
            .map_err(Into::into)
    }

    async fn get_neighbors(&mut self) -> Result<BTreeMap<MacAddr8, u16>, zigbee_nwk::Error> {
        let mut neighbors = BTreeMap::new();

        for index in 0..=u8::MAX {
            match self.transport.get_neighbor(index).await {
                Ok(neighbor) => {
                    neighbors.insert(neighbor.long_id(), neighbor.short_id());
                }
                Err(error) => match error {
                    Error::Status(Status::Ember(Ok(ember::Status::ErrFatal))) => break,
                    error => return Err(error.into()),
                },
            }
        }

        Ok(neighbors)
    }

    async fn route_request(&mut self, radius: u8) -> Result<(), zigbee_nwk::Error> {
        Ok(self
            .transport
            .send_many_to_one_route_request(concentrator::Type::HighRam, radius)
            .await?)
    }

    async fn get_ieee_address(&mut self, pan_id: u16) -> Result<MacAddr8, zigbee_nwk::Error> {
        Ok(self.transport.lookup_eui64_by_node_id(pan_id).await?)
    }

    async fn unicast(
        &mut self,
        pan_id: u16,
        destination_endpoint: Endpoint,
        frame: Frame,
    ) -> Result<u8, zigbee_nwk::Error> {
        let (aps_metadata, payload) = frame.into_parts();
        let tag = self.next_message_tag();
        let message = ByteSizedVec::from_slice(&payload)
            .map_err(io::Error::other)
            .map_err(Error::from)?;
        let aps_frame = self.next_aps_frame(aps_metadata, destination_endpoint, 0x0000);
        let destination = Destination::Direct(pan_id);
        debug!(
            "Sending unicast to: {destination:?}, APS Frame: {aps_frame}, Tag: {tag:#04X}, Message: {:#04X?}",
            message.as_slice()
        );
        Ok(self
            .transport
            .send_unicast(destination, aps_frame, tag, message)
            .await?)
    }

    async fn multicast(
        &mut self,
        group_id: u16,
        hops: u8,
        radius: u8,
        frame: Frame,
    ) -> Result<u8, zigbee_nwk::Error> {
        let (aps_metadata, payload) = frame.into_parts();
        let tag = self.next_message_tag();
        let message = ByteSizedVec::from_slice(&payload)
            .map_err(io::Error::other)
            .map_err(Error::from)?;
        let aps_frame = self.next_aps_frame(aps_metadata, Endpoint::Data, group_id);
        debug!(
            "Sending multicast: Hops: {hops}, Radius: {radius:#04X}, APS Frame: {aps_frame}, Tag: {tag:#04X}, Message: {:#04X?}",
            message.as_slice()
        );
        Ok(self
            .transport
            .send_multicast(aps_frame, hops, radius, tag, message)
            .await?)
    }

    async fn broadcast(
        &mut self,
        pan_id: u16,
        radius: u8,
        frame: Frame,
    ) -> Result<u8, zigbee_nwk::Error> {
        let (aps_metadata, payload) = frame.into_parts();
        let tag = self.next_message_tag();
        let message = ByteSizedVec::from_slice(&payload)
            .map_err(io::Error::other)
            .map_err(Error::from)?;
        let aps_frame = self.next_aps_frame(aps_metadata, Endpoint::Broadcast, 0x0000);
        debug!(
            "Sending broadcast to: {pan_id:#06X}, Radius: {radius:#04X}, APS Frame: {aps_frame}, Tag: {tag:#04X}, Message: {:#04X?}",
            message.as_slice()
        );
        Ok(self
            .transport
            .send_broadcast(pan_id, aps_frame, radius, tag, message)
            .await?)
    }
}
