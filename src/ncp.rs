//! High-level EZSP Network Co-Processor helper.
//!
//! [`Ncp`] wraps an EZSP [`Transport`](crate::Transport) and adds the state
//! needed by host-side Zigbee workflows: endpoint cluster metadata, APS
//! EZSP message tags, scan aggregation, message-sent correlation, and callback
//! dispatch through a background event handler.
//!
//! The type is available without the `apis-saltans` feature. When that feature
//! is enabled, additional implementations adapt [`Ncp`] and [`Builder`] to the
//! `apis_saltans_hw` traits.

use std::num::NonZero;

use log::debug;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::channel;

pub use self::builder::Builder;
pub use self::message::Message;
pub use self::scans::Scans;
use crate::ember::aps::Frame as ApsFrame;
use crate::ember::message::Destination as EmberDestination;
use crate::ember::{Status as EmberStatus, aps};
use crate::error::Status as ErrorStatus;
use crate::ezsp::network::scan;
use crate::parameters::configuration::add_endpoint::Clusters;
use crate::parameters::networking::handler::{EnergyScanResult, NetworkFound};
use crate::types::ByteSizedVec;
use crate::{Error, Messaging, Networking, SharedTransport, Transport};

pub mod builder;
mod message;
mod scans;

// The ZDP profile ID.
const ZDP: u16 = 0x0000;
const STACK_ASSIGNED_APS_SEQUENCE: u8 = 0;
const FIRST_FRAGMENT_INDEX: usize = 0;
const MAX_FRAGMENT_COUNT: usize = u8::MAX as usize;

/// Multicast delivery options for [`Ncp::multicast`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MulticastOptions {
    hops: u8,
    nonmember_radius: u8,
}

impl MulticastOptions {
    /// Creates multicast delivery options.
    #[must_use]
    pub const fn new(hops: u8, nonmember_radius: u8) -> Self {
        Self {
            hops,
            nonmember_radius,
        }
    }

    /// Returns the number of hops for member-node forwarding.
    #[must_use]
    pub const fn hops(self) -> u8 {
        self.hops
    }

    /// Returns the nonmember forwarding radius.
    #[must_use]
    pub const fn nonmember_radius(self) -> u8 {
        self.nonmember_radius
    }
}

/// Host-side helper for an EZSP Network Co-Processor.
///
/// `Ncp<T>` shares exclusive access to the underlying transport. Its methods
/// provide higher-level operations that need callback correlation or local
/// host state, such as scans, outgoing APS message confirmation, and automatic
/// source endpoint selection from the configured endpoint cluster lists.
#[derive(Debug)]
pub struct Ncp<T>
where
    T: Transport,
{
    pub(crate) transport: SharedTransport<T>,
    aps_options: aps::Options,
    message_tag: u8,
    pub(crate) event_handler_proxy: Sender<Message>,
    pub(crate) endpoints: Box<[Clusters]>,
}

impl<T> Ncp<T>
where
    T: Transport,
{
    /// Creates a new `Ncp` with the given transport, event handler, and endpoints.
    ///
    /// `endpoints` must contain the local endpoint cluster metadata in endpoint
    /// order. Outgoing APS helpers use this list to select the source endpoint
    /// for a cluster.
    #[must_use]
    pub fn new(
        transport: impl Into<SharedTransport<T>>,
        aps_options: aps::Options,
        event_handler_proxy: Sender<Message>,
        endpoints: Box<[Clusters]>,
    ) -> Self {
        Self {
            transport: transport.into(),
            aps_options,
            message_tag: 0,
            event_handler_proxy,
            endpoints,
        }
    }

    /// Returns a clone of the shared transport handle.
    #[must_use]
    pub fn shared_transport(&self) -> SharedTransport<T> {
        self.transport.clone()
    }

    /// Returns the next message tag and increments the internal counter.
    pub(crate) const fn next_message_tag(&mut self) -> u8 {
        let tag = self.message_tag;
        self.message_tag = self.message_tag.wrapping_add(1);
        tag
    }

    /// Build an outgoing EZSP APS frame using the configured APS options.
    ///
    /// EZSP assigns the APS sequence when a send command is accepted. The
    /// sequence field in the command payload is therefore initialized with a
    /// placeholder value.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NoMatchingSourceEndpoint`] when no local endpoint advertises `cluster_id`.
    pub(crate) fn aps_frame(
        &self,
        profile_id: u16,
        cluster_id: u16,
        destination_endpoint: u8,
        group_id: u16,
    ) -> Result<aps::Frame, Error> {
        Ok(aps::Frame::new(
            profile_id,
            cluster_id,
            self.source_endpoint(profile_id, cluster_id)?,
            destination_endpoint,
            self.aps_options,
            group_id,
            STACK_ASSIGNED_APS_SEQUENCE,
        ))
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

    /// Sends a termination request to the background event handler.
    ///
    /// # Errors
    ///
    /// Returns [`SendError`] if the termination
    /// request cannot be sent to the message handler.
    pub async fn terminate(self) -> Result<(), SendError<Message>> {
        self.event_handler_proxy.send(Message::Terminate).await
    }
}

impl<T> Ncp<T>
where
    T: Messaging + Transport,
{
    /// Sends a unicast APS message and waits for its `messageSent` status.
    ///
    /// Payloads larger than the EZSP maximum APS payload length are fragmented
    /// for unicast delivery. The stack-assigned APS sequence from the first
    /// fragment is reused for follow-up fragments, matching EZSP host
    /// fragmentation behavior.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if no matching source endpoint exists, payload
    /// fragmentation would exceed 255 fragments, registering the message tag
    /// fails, sending an EZSP command fails, or receiving a successful
    /// `messageSent` callback fails.
    pub async fn unicast(
        &mut self,
        short_id: u16,
        profile_id: u16,
        cluster_id: u16,
        destination_endpoint: u8,
        payload: impl AsRef<[u8]>,
    ) -> Result<(), Error> {
        let payload = payload.as_ref();
        let aps_frame = self.aps_frame(profile_id, cluster_id, destination_endpoint, 0)?;
        let destination = EmberDestination::Direct(short_id);
        let maximum_payload_length = {
            let mut transport = self.transport.lock().await;
            usize::from(transport.maximum_payload_length().await?)
        };

        if payload.len() <= maximum_payload_length {
            self.send_unicast_fragment(destination, aps_frame, payload)
                .await?;
        } else {
            self.send_fragmented_unicast(destination, aps_frame, payload, maximum_payload_length)
                .await?;
        }

        Ok(())
    }

    /// Sends a multicast APS message and waits for its `messageSent` status.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if no matching source endpoint exists, the payload
    /// is larger than the EZSP maximum APS payload length, registering the
    /// message tag fails, sending the EZSP command fails, or receiving a
    /// successful `messageSent` callback fails.
    pub async fn multicast(
        &mut self,
        group_id: u16,
        options: MulticastOptions,
        profile_id: u16,
        cluster_id: u16,
        destination_endpoint: u8,
        payload: impl AsRef<[u8]>,
    ) -> Result<(), Error> {
        let payload = payload.as_ref();
        let aps_frame = self.aps_frame(profile_id, cluster_id, destination_endpoint, group_id)?;
        let tag = self.next_message_tag();
        let message = self.reject_oversized_payload(payload).await?;

        debug!(
            "Sending multicast: Hops: {}, Radius: {:#04X}, APS Frame: {aps_frame}, Tag: {tag:#04X}, Message: {:#04X?}",
            options.hops(),
            options.nonmember_radius(),
            message.as_slice()
        );

        let (tx, rx) = channel();
        self.event_handler_proxy
            .send(Message::Sent { tag, sender: tx })
            .await?;

        let _sequence = {
            let mut transport = self.transport.lock().await;
            transport
                .send_multicast(
                    aps_frame,
                    options.hops(),
                    options.nonmember_radius(),
                    tag,
                    message,
                )
                .await?
        };

        match rx.await? {
            Ok(EmberStatus::Success) => Ok(()),
            other => Err(Error::Status(ErrorStatus::Ember(other))),
        }
    }

    /// Sends a broadcast APS message and waits for its `messageSent` status.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if no matching source endpoint exists, the payload
    /// is larger than the EZSP maximum APS payload length, registering the
    /// message tag fails, sending the EZSP command fails, or receiving a
    /// successful `messageSent` callback fails.
    pub async fn broadcast(
        &mut self,
        short_id: u16,
        radius: u8,
        profile_id: u16,
        cluster_id: u16,
        destination_endpoint: u8,
        payload: impl AsRef<[u8]>,
    ) -> Result<(), Error> {
        let payload = payload.as_ref();
        let aps_frame = self.aps_frame(profile_id, cluster_id, destination_endpoint, 0)?;
        let tag = self.next_message_tag();
        let message = self.reject_oversized_payload(payload).await?;

        debug!(
            "Sending broadcast to: {short_id:#06X}, Radius: {radius:#04X}, APS Frame: {aps_frame}, Tag: {tag:#04X}, Message: {:#04X?}",
            message.as_slice()
        );

        let (tx, rx) = channel();
        self.event_handler_proxy
            .send(Message::Sent { tag, sender: tx })
            .await?;

        let _sequence = {
            let mut transport = self.transport.lock().await;
            transport
                .send_broadcast(short_id, aps_frame, radius, tag, message)
                .await?
        };

        match rx.await? {
            Ok(EmberStatus::Success) => Ok(()),
            other => Err(Error::Status(ErrorStatus::Ember(other))),
        }
    }

    async fn send_fragmented_unicast(
        &mut self,
        destination: EmberDestination,
        aps_frame: ApsFrame,
        payload: &[u8],
        maximum_payload_length: usize,
    ) -> Result<(), Error> {
        let fragment_count = fragment_count(payload.len(), maximum_payload_length)?;
        let mut sequence = None;

        for (index, chunk) in payload.chunks(maximum_payload_length).enumerate() {
            let mut fragment = aps_frame.clone();
            fragment.enable_retry();

            if index == FIRST_FRAGMENT_INDEX {
                fragment.set_first_fragment(fragment_count);
            } else {
                let sequence = sequence.expect("first fragment sets the APS sequence");
                let index = u8::try_from(index).expect("fragment count is limited to u8::MAX");
                fragment.set_sequence(sequence);
                fragment.set_followup_fragment(
                    NonZero::new(index).expect("follow-up fragment index is non-zero"),
                );
            }

            let fragment_sequence = self
                .send_unicast_fragment(destination, fragment, chunk)
                .await?;

            if index == FIRST_FRAGMENT_INDEX {
                sequence.replace(fragment_sequence);
            }
        }

        Ok(())
    }

    async fn send_unicast_fragment(
        &mut self,
        destination: EmberDestination,
        aps_frame: ApsFrame,
        payload: &[u8],
    ) -> Result<u8, Error> {
        let tag = self.next_message_tag();
        let message = byte_sized_payload(payload)?;

        debug!(
            "Sending unicast to: {destination:?}, APS Frame: {aps_frame}, Tag: {tag:#04X}, Message: {:#04X?}",
            message.as_slice()
        );

        let (tx, rx) = channel();
        self.event_handler_proxy
            .send(Message::Sent { tag, sender: tx })
            .await?;

        let sequence = {
            let mut transport = self.transport.lock().await;
            transport
                .send_unicast(destination, aps_frame, tag, message)
                .await?
        };

        match rx.await? {
            Ok(EmberStatus::Success) => Ok(sequence),
            other => Err(Error::Status(ErrorStatus::Ember(other))),
        }
    }

    async fn reject_oversized_payload(&self, payload: &[u8]) -> Result<ByteSizedVec<u8>, Error> {
        let maximum_payload_length = {
            let mut transport = self.transport.lock().await;
            usize::from(transport.maximum_payload_length().await?)
        };

        if payload.len() > maximum_payload_length {
            Err(message_too_long())
        } else {
            byte_sized_payload(payload)
        }
    }
}

impl<T> Ncp<T>
where
    T: Networking + Transport,
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
        {
            let mut transport = self.transport.lock().await;
            transport
                .start_scan(scan::Type::ActiveScan, channel_mask, duration)
                .await?;
            drop(transport);
        }
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
        {
            let mut transport = self.transport.lock().await;
            transport
                .start_scan(scan::Type::EnergyScan, channel_mask, duration)
                .await?;
            drop(transport);
        }
        Ok(rx.await?)
    }
}

fn byte_sized_payload(payload: &[u8]) -> Result<ByteSizedVec<u8>, Error> {
    ByteSizedVec::from_slice(payload).map_err(|_| message_too_long())
}

fn fragment_count(payload_length: usize, maximum_payload_length: usize) -> Result<u8, Error> {
    if maximum_payload_length == 0 {
        return Err(message_too_long());
    }

    let fragments = payload_length.div_ceil(maximum_payload_length);

    if fragments > MAX_FRAGMENT_COUNT {
        return Err(message_too_long());
    }

    u8::try_from(fragments).map_err(|_| message_too_long())
}

const fn message_too_long() -> Error {
    Error::Status(ErrorStatus::Ember(Ok(EmberStatus::MessageTooLong)))
}
