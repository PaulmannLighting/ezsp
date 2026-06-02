//! Network manager for Zigbee networks.

use std::collections::BTreeMap;
use std::io;
use std::time::Duration;

use log::{debug, info};
use macaddr::MacAddr8;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::sync::oneshot;
use tokio::task::{JoinError, JoinHandle};
use zigbee::{Endpoint, Profile};
use zigbee_hw::{Event, Frame, Metadata, NcpDriver};

use self::builder::Builder;
use self::event_mux::Message;
use crate::ember::message::Destination;
use crate::ember::{aps, concentrator};
use crate::error::Status;
use crate::ezsp::network::scan;
use crate::types::ByteSizedVec;
use crate::{Configuration, Error, Messaging, Networking, Security, Utilities, ember};

mod builder;
mod event_mux;

/// Network manager for Zigbee networks.
#[derive(Debug)]
pub struct EzspNetworkManager<T> {
    transport: T,
    profile: Profile,
    aps_options: aps::Options,
    message_tag: u8,
    aps_seq: u8,
    transaction_seq: u8,
    message_handler_proxy: Sender<Message>,
    message_handler_handle: JoinHandle<()>,
}

impl<T> EzspNetworkManager<T> {
    /// Creates a new `NetworkManager` with the given transport.
    #[must_use]
    pub(crate) const fn new(
        transport: T,
        profile: Profile,
        aps_options: aps::Options,
        message_handler_proxy: Sender<Message>,
        message_handler_handle: JoinHandle<()>,
    ) -> Self {
        Self {
            transport,
            profile,
            aps_options,
            message_tag: 0,
            aps_seq: 0,
            transaction_seq: 0,
            message_handler_proxy,
            message_handler_handle,
        }
    }

    /// Creates a new `Builder` for constructing a `NetworkManager`.
    #[must_use]
    pub fn build(transport: T) -> Builder<T> {
        Builder::new(transport)
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

    /// Creates a new APS frame with the given parameters.
    fn next_aps_frame(
        &mut self,
        aps_metadata: &Metadata,
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

    /// Terminate the network manager.
    ///
    /// # Errors
    ///
    /// Returns a [`JoinError`] if joining the message handler fails.
    ///
    /// # Panics
    ///
    /// This method may panic, if the termination signal cannot be sent to the message handler.
    pub async fn terminate(self) -> Result<T, JoinError> {
        self.message_handler_proxy
            .send(Message::Terminate)
            .await
            .expect("Failed to send terminate message to message handler actor");
        self.message_handler_handle.await.map(|()| self.transport)
    }
}

impl<T> NcpDriver for EzspNetworkManager<T>
where
    T: Configuration + Security + Messaging + Networking + Utilities + Send + Sync,
{
    fn next_transaction_seq(&mut self) -> u8 {
        let seq = self.transaction_seq;
        self.transaction_seq = self.transaction_seq.wrapping_add(1);
        seq
    }

    async fn get_short_id(&mut self) -> Result<u16, zigbee_hw::Error> {
        Err(zigbee_hw::Error::NotImplemented)
    }

    async fn scan_networks(
        &mut self,
        channel_mask: u32,
        duration: u8,
    ) -> Result<Vec<zigbee_hw::FoundNetwork>, zigbee_hw::Error> {
        let (tx, rx) = oneshot::channel();
        self.message_handler_proxy.send(tx.into()).await?;
        self.transport
            .start_scan(scan::Type::ActiveScan, channel_mask, duration)
            .await?;
        Ok(rx.await?)
    }

    async fn scan_channels(
        &mut self,
        channel_mask: u32,
        duration: u8,
    ) -> Result<Vec<zigbee_hw::ScannedChannel>, zigbee_hw::Error> {
        let (tx, rx) = oneshot::channel();
        self.message_handler_proxy.send(tx.into()).await?;
        self.transport
            .start_scan(scan::Type::EnergyScan, channel_mask, duration)
            .await?;
        Ok(rx.await?)
    }

    async fn allow_joins(&mut self, duration: Duration) -> Result<(), zigbee_hw::Error> {
        info!("Allowing joins for {} seconds.", duration.as_secs());
        self.transport
            .permit_joining(u8::try_from(duration.as_secs()).unwrap_or(u8::MAX).into())
            .await
            .map_err(Into::into)
    }

    async fn get_neighbors(&mut self) -> Result<BTreeMap<MacAddr8, u16>, zigbee_hw::Error> {
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

    async fn route_request(&mut self, radius: u8) -> Result<(), zigbee_hw::Error> {
        Ok(self
            .transport
            .send_many_to_one_route_request(concentrator::Type::HighRam, radius)
            .await?)
    }

    async fn get_ieee_address(&mut self, short_id: u16) -> Result<MacAddr8, zigbee_hw::Error> {
        Ok(self.transport.lookup_eui64_by_node_id(short_id).await?)
    }

    async fn unicast(
        &mut self,
        short_id: u16,
        destination_endpoint: Endpoint,
        frame: Frame,
    ) -> Result<u8, zigbee_hw::Error> {
        let (aps_metadata, payload) = frame.into_parts();
        let tag = self.next_message_tag();
        let message = ByteSizedVec::from_slice(&payload)
            .map_err(io::Error::other)
            .map_err(Error::from)?;
        let aps_frame = self.next_aps_frame(&aps_metadata, destination_endpoint, 0x0000);
        let destination = Destination::Direct(short_id);
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
    ) -> Result<u8, zigbee_hw::Error> {
        let (aps_metadata, payload) = frame.into_parts();
        let tag = self.next_message_tag();
        let message = ByteSizedVec::from_slice(&payload)
            .map_err(io::Error::other)
            .map_err(Error::from)?;
        let aps_frame = self.next_aps_frame(&aps_metadata, Endpoint::Data, group_id);
        debug!(
            "Sending multicast: Hops: {hops}, Radius: {radius:#04X}, APS Frame: {aps_frame}, Tag: {tag:#04X}, Message: {:#04X?}",
            message.as_slice()
        );
        Ok(self
            .transport
            .send_multicast(aps_frame, hops, radius, tag, message)
            .await?)
    }

    async fn subscribe(&mut self, buffer: usize) -> Result<Receiver<Event>, zigbee_hw::Error> {
        let (tx, rx) = channel(buffer);
        self.message_handler_proxy.send(tx.into()).await?;
        Ok(rx)
    }

    async fn broadcast(
        &mut self,
        short_id: u16,
        radius: u8,
        frame: Frame,
    ) -> Result<u8, zigbee_hw::Error> {
        let (aps_metadata, payload) = frame.into_parts();
        let tag = self.next_message_tag();
        let message = ByteSizedVec::from_slice(&payload)
            .map_err(io::Error::other)
            .map_err(Error::from)?;
        let aps_frame = self.next_aps_frame(&aps_metadata, Endpoint::Broadcast, 0x0000);
        debug!(
            "Sending broadcast to: {short_id:#06X}, Radius: {radius:#04X}, APS Frame: {aps_frame}, Tag: {tag:#04X}, Message: {:#04X?}",
            message.as_slice()
        );
        Ok(self
            .transport
            .send_broadcast(short_id, aps_frame, radius, tag, message)
            .await?)
    }
}
