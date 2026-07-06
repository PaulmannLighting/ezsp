//! High-level EZSP Network Co-Processor helper.
//!
//! [`Ncp`] wraps an EZSP [`Transport`](crate::Transport) and adds the state
//! needed by host-side Zigbee workflows: endpoint cluster metadata, APS
//! sequence numbers, EZSP message tags, transaction sequence numbers, scan
//! aggregation, message-sent correlation, and callback dispatch through a
//! background event handler.
//!
//! The type is available without the `apis-saltans` feature. When that feature
//! is enabled, additional implementations adapt [`Ncp`] and [`Builder`] to the
//! `apis_saltans_hw` traits.

use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use log::debug;
use macaddr::MacAddr8;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot::channel;
use tokio::task::{JoinError, JoinHandle};

pub use self::builder::Builder;
pub use self::message::Message;
pub use self::scans::Scans;
use crate::ember::aps;
use crate::ember::message::Destination;
use crate::error::Status;
use crate::ezsp::network::scan;
use crate::parameters::configuration::add_endpoint::Clusters;
use crate::parameters::networking::handler::{EnergyScanResult, NetworkFound};
use crate::types::ByteSizedVec;
use crate::{Callback, Error, Messaging, Networking, ember};

mod builder;
mod message;
mod scans;

// The ZDP profile ID.
const ZDP: u16 = 0x0000;

/// Host-side helper for an EZSP Network Co-Processor.
///
/// `Ncp<T>` owns the underlying transport and dereferences to it, so all normal
/// EZSP command traits remain available. Its own methods provide higher-level
/// operations that need callback correlation or local host state, such as
/// scans, outgoing APS message confirmation, and automatic source endpoint
/// selection from the configured endpoint cluster lists.
#[derive(Debug)]
pub struct Ncp<T> {
    transport: T,
    aps_options: aps::Options,
    message_tag: u8,
    aps_seq: u8,
    event_handler_proxy: Sender<Message>,
    event_handler_handle: JoinHandle<()>,
    endpoints: Box<[Clusters]>,
}

impl<T> Ncp<T> {
    /// Creates a new `Ncp` with the given transport, event handler, and endpoints.
    ///
    /// `endpoints` must contain the local endpoint cluster metadata in endpoint
    /// order. Outgoing APS helpers use this list to select the source endpoint
    /// for a cluster.
    #[must_use]
    pub const fn new(
        transport: T,
        aps_options: aps::Options,
        event_handler_proxy: Sender<Message>,
        event_handler_handle: JoinHandle<()>,
        endpoints: Box<[Clusters]>,
    ) -> Self {
        Self {
            transport,
            aps_options,
            message_tag: 0,
            aps_seq: 0,
            event_handler_proxy,
            event_handler_handle,
            endpoints,
        }
    }

    /// Creates a new [`Builder`] for constructing an [`Ncp`].
    #[must_use]
    pub const fn build(transport: T, callbacks: Receiver<Callback>) -> Builder<T> {
        Builder::new(transport, callbacks)
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

    /// Returns the first local endpoint that lists the given cluster as an output cluster.
    ///
    /// Endpoints are stored in the same order they were supplied to the builder.
    /// The returned endpoint number is one-based, matching the EZSP endpoint
    /// numbering used for outgoing APS frames.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NoMatchingSourceEndpoint`] when no configured local
    /// endpoint advertises `cluster_id` as an output cluster.
    pub fn source_endpoint(&self, profile_id: u16, cluster_id: u16) -> Result<u8, Error> {
        if profile_id == ZDP {
            return Ok(0);
        }

        self.endpoints
            .iter()
            .enumerate()
            .find_map(|(index, endpoint)| {
                if endpoint.output_clusters().contains(&cluster_id) {
                    index.checked_add(1).and_then(|index| index.try_into().ok())
                } else {
                    None
                }
            })
            .ok_or(Error::NoMatchingSourceEndpoint(cluster_id))
    }

    /// Creates a new APS frame with the given parameters.
    const fn next_aps_frame(
        &mut self,
        profile_id: u16,
        cluster_id: u16,
        source_endpoint: u8,
        destination_endpoint: u8,
        group_id: u16,
    ) -> aps::Frame {
        aps::Frame::new(
            profile_id,
            cluster_id,
            source_endpoint,
            destination_endpoint,
            self.aps_options,
            group_id,
            self.next_aps_seq(),
        )
    }

    /// Terminates the background event handler and returns the underlying transport.
    ///
    /// # Errors
    ///
    /// Returns a [`JoinError`] if joining the message handler fails.
    ///
    /// # Panics
    ///
    /// This method may panic if the termination signal cannot be sent to the message handler.
    pub async fn terminate(self) -> Result<T, JoinError> {
        self.event_handler_proxy
            .send(Message::Terminate)
            .await
            .expect("Failed to send terminate message to message handler actor");
        self.event_handler_handle.await.map(|()| self.transport)
    }
}

impl<T> Ncp<T>
where
    T: Networking + Send + Sync,
{
    /// Starts an active network scan and returns all `networkFound` callback results.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if registering the scan, sending `startScan`, or
    /// receiving the scan result fails.
    pub async fn scan_networks(
        &mut self,
        channel_mask: u32,
        duration: u8,
    ) -> Result<Vec<NetworkFound>, Error> {
        let (tx, rx) = channel();
        self.event_handler_proxy.send(tx.into()).await?;
        self.transport
            .start_scan(scan::Type::ActiveScan, channel_mask, duration)
            .await?;
        Ok(rx.await?)
    }

    /// Starts an energy scan and returns all `energyScanResult` callback results.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if registering the scan, sending `startScan`, or
    /// receiving the scan result fails.
    pub async fn scan_channels(
        &mut self,
        channel_mask: u32,
        duration: u8,
    ) -> Result<Vec<EnergyScanResult>, Error> {
        let (tx, rx) = channel();
        self.event_handler_proxy.send(tx.into()).await?;
        self.transport
            .start_scan(scan::Type::EnergyScan, channel_mask, duration)
            .await?;
        Ok(rx.await?)
    }

    /// Reads the neighbor table as a map from IEEE address to short ID.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if an EZSP command fails with anything other than
    /// the stack's end-of-table `ErrFatal` sentinel.
    pub async fn get_neighbors(&mut self) -> Result<BTreeMap<MacAddr8, u16>, Error> {
        let mut neighbors = BTreeMap::new();

        for index in 0..=u8::MAX {
            match self.transport.get_neighbor(index).await {
                Ok(neighbor) => {
                    neighbors.insert(neighbor.long_id(), neighbor.short_id());
                }
                Err(error) => match error {
                    Error::Status(Status::Ember(Ok(ember::Status::ErrFatal))) => break,
                    error => return Err(error),
                },
            }
        }

        Ok(neighbors)
    }
}

impl<T> Ncp<T>
where
    T: Messaging + Send + Sync,
{
    /// Sends a unicast APS message and returns a future for its `messageSent` status.
    ///
    /// The APS frame uses `profile_id`, `cluster_id`, and
    /// `destination_endpoint` directly. Its source endpoint is selected from
    /// the first configured local endpoint that lists `cluster_id` as an output
    /// cluster.
    ///
    /// The returned response receiver resolves to the APS sequence number if
    /// the stack reports `EMBER_SUCCESS`.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if no matching source endpoint exists, registering
    /// the message tag fails, or sending the EZSP command fails.
    pub async fn unicast(
        &mut self,
        short_id: u16,
        profile_id: u16,
        cluster_id: u16,
        destination_endpoint: u8,
        message: ByteSizedVec<u8>,
    ) -> Result<u8, Error> {
        let tag = self.next_message_tag();
        let aps_frame = self.next_aps_frame(
            profile_id,
            cluster_id,
            self.source_endpoint(profile_id, cluster_id)?,
            destination_endpoint,
            0x0000,
        );
        let destination = Destination::Direct(short_id);

        debug!(
            "Sending unicast to: {destination:?}, APS Frame: {aps_frame}, Tag: {tag:#04X}, Message: {:#04X?}",
            message.as_slice()
        );

        let (tx, rx) = channel();

        self.event_handler_proxy
            .send(Message::Sent { tag, sender: tx })
            .await?;

        let seq = self
            .transport
            .send_unicast(destination, aps_frame, tag, message)
            .await?;

        match rx.await? {
            Ok(ember::Status::Success) => Ok(seq),
            other => Err(Error::Status(Status::Ember(other))),
        }
    }

    /// Sends a multicast APS message and returns a future for its `messageSent` status.
    ///
    /// The APS frame uses `profile_id`, `cluster_id`, `destination_endpoint`,
    /// and `group_id` directly. Its source endpoint is selected from the first
    /// configured local endpoint that lists `cluster_id` as an output cluster.
    ///
    /// The returned response receiver resolves to the APS sequence number if
    /// the stack reports `EMBER_SUCCESS`.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if no matching source endpoint exists, registering
    /// the message tag fails, or sending the EZSP command fails.
    #[expect(clippy::too_many_arguments)]
    pub async fn multicast(
        &mut self,
        group_id: u16,
        hops: u8,
        radius: u8,
        profile_id: u16,
        cluster_id: u16,
        destination_endpoint: u8,
        message: ByteSizedVec<u8>,
    ) -> Result<u8, Error> {
        let tag = self.next_message_tag();
        let aps_frame = self.next_aps_frame(
            profile_id,
            cluster_id,
            self.source_endpoint(profile_id, cluster_id)?,
            destination_endpoint,
            group_id,
        );

        debug!(
            "Sending multicast: Hops: {hops}, Radius: {radius:#04X}, APS Frame: {aps_frame}, Tag: {tag:#04X}, Message: {:#04X?}",
            message.as_slice()
        );

        let (tx, rx) = channel();

        self.event_handler_proxy
            .send(Message::Sent { tag, sender: tx })
            .await?;

        let seq = self
            .transport
            .send_multicast(aps_frame, hops, radius, tag, message)
            .await?;

        match rx.await? {
            Ok(ember::Status::Success) => Ok(seq),
            other => Err(Error::Status(Status::Ember(other))),
        }
    }

    /// Sends a broadcast APS message and waits for its `messageSent` status.
    ///
    /// The APS frame uses `profile_id`, `cluster_id`, and
    /// `destination_endpoint` directly. Its source endpoint is selected from
    /// the first configured local endpoint that lists `cluster_id` as an output
    /// cluster.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if no matching source endpoint exists, registering
    /// the message tag fails, sending the EZSP command fails, or receiving a
    /// successful `messageSent` callback fails.
    pub async fn broadcast(
        &mut self,
        short_id: u16,
        radius: u8,
        profile_id: u16,
        cluster_id: u16,
        destination_endpoint: u8,
        message: ByteSizedVec<u8>,
    ) -> Result<u8, Error> {
        let tag = self.next_message_tag();
        let aps_frame = self.next_aps_frame(
            profile_id,
            cluster_id,
            self.source_endpoint(profile_id, cluster_id)?,
            destination_endpoint,
            0x0000,
        );
        debug!(
            "Sending broadcast to: {short_id:#06X}, Radius: {radius:#04X}, APS Frame: {aps_frame}, Tag: {tag:#04X}, Message: {:#04X?}",
            message.as_slice()
        );

        let (tx, rx) = channel();

        self.event_handler_proxy
            .send(Message::Sent { tag, sender: tx })
            .await?;

        let seq = self
            .transport
            .send_broadcast(short_id, aps_frame, radius, tag, message)
            .await?;

        match rx.await? {
            Ok(ember::Status::Success) => Ok(seq),
            other => Err(Error::Status(Status::Ember(other))),
        }
    }
}

#[cfg(feature = "ashv2")]
impl Ncp<crate::uart::Uart> {
    /// Creates a new [`Builder`] backed by an `ASHv2` UART transport.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the building of the UART fails.
    pub fn ashv2<P>(serial_port: P) -> Result<Builder<crate::uart::Uart>, Error>
    where
        P: ashv2::SerialPort + ashv2::TryCloneNative + Sync + 'static,
    {
        Builder::ashv2(serial_port)
    }
}

impl<T> Deref for Ncp<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.transport
    }
}

impl<T> DerefMut for Ncp<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.transport
    }
}
