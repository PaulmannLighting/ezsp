//! High-level EZSP Network Co-Processor helper.
//!
//! [`Ncp`] wraps a connected EZSP communicator and adds the state needed by
//! host-side Zigbee workflows: endpoint cluster metadata, APS message tags,
//! scan aggregation, message-sent correlation, and callback dispatch through a
//! background event handler.
//!
//! [`Builder`] negotiates the protocol version through caller-spawned transport
//! actors, configures the stack, registers endpoints, and returns an [`Ncp`]
//! together with callback-processing futures for the caller to spawn.
//! [`Startup`] records whether the builder should restore the NCP's persisted
//! network or explicitly form a new network. With the
//! `apis-saltans` feature, `Ncp` also implements
//! `apis_saltans_hw::Driver` for suitable communicators and gains conversions
//! between EZSP and `apis-saltans` endpoint, scan, APS, and event types.

use std::num::NonZero;

use log::debug;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::channel;

pub use self::builder::Builder;
pub use self::endpoint::Endpoint;
pub use self::event_handler::EventHandler;
pub use self::initialization_parameters::InitializationParameters;
pub use self::message::Message;
pub use self::multicast_options::MulticastOptions;
pub use self::network_credentials::NetworkCredentials;
pub use self::scans::Scans;
pub use self::stack_response::StackResponse;
pub use self::startup::Startup;
use crate::ember::aps::Frame as ApsFrame;
use crate::ember::message::Destination as EmberDestination;
use crate::ember::{Status as EmberStatus, aps};
use crate::error::Status as ErrorStatus;
use crate::ezsp::network::scan;
use crate::parameters::networking::handler::{EnergyScanResult, NetworkFound};
use crate::types::ByteSizedVec;
use crate::{Connection, Error, Messaging, Networking};

mod await_event;
pub mod builder;
mod endpoint;
mod event_handler;
mod initialization_parameters;
mod message;
mod multicast_options;
mod network_credentials;
mod scans;
mod stack_response;
mod startup;

// The ZDP profile ID.
const ZDP: u16 = 0x0000;
const STACK_ASSIGNED_APS_SEQUENCE: u8 = 0;
const FIRST_FRAGMENT_INDEX: usize = 0;
const MAX_FRAGMENT_COUNT: usize = u8::MAX as usize;

/// Host-side helper for an EZSP Network Co-Processor.
///
/// `Ncp` owns a cloneable [`Connection`] actor handle. Its methods provide
/// higher-level operations
/// that need callback correlation or local host state, such as scans, outgoing
/// APS message confirmation, and automatic source endpoint selection from the
/// configured endpoint cluster lists. The builder gives another clone of the
/// connected handle to the background [`EventHandler`].
#[derive(Debug)]
pub struct Ncp {
    pub(crate) connection: Connection,
    pub(crate) endpoints: Box<[Endpoint]>,
    event_handler_handle: Sender<Message>,
    aps_options: aps::Options,
    message_tag: u8,
}

impl Ncp {
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

    /// Returns the lowest-numbered local endpoint that advertises an output cluster.
    ///
    /// ZDP messages always use endpoint zero. For other profiles, the endpoint
    /// registry is searched in ascending endpoint-number order and the first
    /// endpoint containing `cluster_id` in its output-cluster set is returned.
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
            .find_map(|endpoint| {
                if endpoint.output_clusters.contains(&cluster_id) {
                    Some(endpoint.id)
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
        self.event_handler_handle.send(Message::Terminate).await
    }

    /// Registers endpoints and constructs a high-level NCP helper.
    ///
    /// Each endpoint is registered on the NCP before the value is returned.
    /// The supplied event-handler sender must feed the same callback handler
    /// that receives callbacks for `transport`, because scans and APS send
    /// confirmations are correlated through that channel.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any endpoint registration command fails.
    pub async fn new(
        mut connection: Connection,
        endpoints: Box<[Endpoint]>,
        event_handler_handle: Sender<Message>,
        aps_options: aps::Options,
    ) -> Result<Self, Error> {
        for endpoint in endpoints.iter().cloned() {
            endpoint.add_to(&mut connection).await?;
        }

        Ok(Self {
            connection,
            endpoints,
            event_handler_handle,
            aps_options,
            message_tag: 0,
        })
    }

    /// Starts a unicast APS send and returns its deferred [`StackResponse`].
    ///
    /// Payloads larger than the EZSP maximum APS payload length are fragmented
    /// for unicast delivery. The stack-assigned APS sequence from the first
    /// fragment is reused for follow-up fragments, matching EZSP host
    /// fragmentation behavior. Responses for all non-final fragments are
    /// awaited before this method returns the response for the final fragment.
    /// Await the returned [`StackResponse`] to confirm that final response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if no matching source endpoint exists, payload
    /// fragmentation would exceed 255 fragments, registering a message tag or
    /// sending an EZSP command fails, or a non-final fragment's stack response
    /// reports failure. Errors reported by the returned [`StackResponse`] occur
    /// when that value is awaited.
    pub async fn unicast(
        &mut self,
        short_id: u16,
        profile_id: u16,
        cluster_id: u16,
        destination_endpoint: u8,
        payload: impl AsRef<[u8]>,
    ) -> Result<StackResponse, Error> {
        let payload = payload.as_ref();
        let aps_frame = self.aps_frame(profile_id, cluster_id, destination_endpoint, 0)?;
        let destination = EmberDestination::Direct(short_id);
        let maximum_payload_length = usize::from(self.connection.maximum_payload_length().await?);

        let stack_response = if payload.len() <= maximum_payload_length {
            let (stack_response, _seq) = self
                .send_unicast_fragment(destination, aps_frame, payload)
                .await?;
            stack_response
        } else {
            self.send_fragmented_unicast(destination, aps_frame, payload, maximum_payload_length)
                .await?
        };

        Ok(stack_response)
    }

    /// Starts a multicast APS send.
    ///
    /// Returns the deferred [`StackResponse`] and the APS sequence assigned by
    /// the NCP. Await the response to confirm the matching `messageSent`
    /// callback.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if no matching source endpoint exists, the payload
    /// is larger than the EZSP maximum APS payload length, registering the
    /// message tag fails, or sending the EZSP command fails. Errors reported by
    /// the returned [`StackResponse`] occur when that value is awaited.
    pub async fn multicast(
        &mut self,
        group_id: u16,
        options: MulticastOptions,
        profile_id: u16,
        cluster_id: u16,
        destination_endpoint: u8,
        payload: impl AsRef<[u8]>,
    ) -> Result<(StackResponse, u8), Error> {
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
        self.event_handler_handle
            .send(Message::Sent { tag, sender: tx })
            .await?;

        let sequence = self
            .connection
            .send_multicast(
                aps_frame,
                options.hops(),
                options.nonmember_radius(),
                tag,
                message,
            )
            .await?;

        Ok((rx.into(), sequence))
    }

    /// Starts a broadcast APS send and returns its deferred [`StackResponse`].
    ///
    /// Await the returned response to confirm the matching `messageSent`
    /// callback.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if no matching source endpoint exists, the payload
    /// is larger than the EZSP maximum APS payload length, registering the
    /// message tag fails, or sending the EZSP command fails. Errors reported by
    /// the returned [`StackResponse`] occur when that value is awaited.
    pub async fn broadcast(
        &mut self,
        short_id: u16,
        radius: u8,
        profile_id: u16,
        cluster_id: u16,
        destination_endpoint: u8,
        payload: impl AsRef<[u8]>,
    ) -> Result<StackResponse, Error> {
        let payload = payload.as_ref();
        let aps_frame = self.aps_frame(profile_id, cluster_id, destination_endpoint, 0)?;
        let tag = self.next_message_tag();
        let message = self.reject_oversized_payload(payload).await?;

        debug!(
            "Sending broadcast to: {short_id:#06X}, Radius: {radius:#04X}, APS Frame: {aps_frame}, Tag: {tag:#04X}, Message: {:#04X?}",
            message.as_slice()
        );

        let (tx, rx) = channel();
        self.event_handler_handle
            .send(Message::Sent { tag, sender: tx })
            .await?;

        self.connection
            .send_broadcast(short_id, aps_frame, radius, tag, message)
            .await?;

        Ok(rx.into())
    }

    async fn send_fragmented_unicast(
        &mut self,
        destination: EmberDestination,
        aps_frame: ApsFrame,
        payload: &[u8],
        maximum_payload_length: usize,
    ) -> Result<StackResponse, Error> {
        let fragment_count = fragment_count(payload.len(), maximum_payload_length)?;
        let mut last_stack_response = None;
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

            let (stack_response, seq) = self
                .send_unicast_fragment(destination, fragment, chunk)
                .await?;

            if index == FIRST_FRAGMENT_INDEX {
                sequence.replace(seq);
            }

            if let Some(stack_response) = last_stack_response.replace(stack_response) {
                stack_response.await?;
            }
        }

        Ok(last_stack_response
            .take()
            .expect("fragmented send always produces a final stack response"))
    }

    async fn send_unicast_fragment(
        &mut self,
        destination: EmberDestination,
        aps_frame: ApsFrame,
        payload: &[u8],
    ) -> Result<(StackResponse, u8), Error> {
        let tag = self.next_message_tag();
        let message = byte_sized_payload(payload)?;

        debug!(
            "Sending unicast to: {destination:?}, APS Frame: {aps_frame}, Tag: {tag:#04X}, Message: {:#04X?}",
            message.as_slice()
        );

        let (tx, rx) = channel();
        self.event_handler_handle
            .send(Message::Sent { tag, sender: tx })
            .await?;

        let sequence = self
            .connection
            .send_unicast(destination, aps_frame, tag, message)
            .await?;

        Ok((rx.into(), sequence))
    }

    async fn reject_oversized_payload(
        &mut self,
        payload: &[u8],
    ) -> Result<ByteSizedVec<u8>, Error> {
        let maximum_payload_length = usize::from(self.connection.maximum_payload_length().await?);

        if payload.len() > maximum_payload_length {
            Err(message_too_long())
        } else {
            byte_sized_payload(payload)
        }
    }

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
        self.event_handler_handle.send(tx.into()).await?;
        self.connection
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
        self.event_handler_handle.send(tx.into()).await?;
        self.connection
            .start_scan(scan::Type::EnergyScan, channel_mask, duration)
            .await?;
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
