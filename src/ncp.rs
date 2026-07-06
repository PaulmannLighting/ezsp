//! Network manager for Zigbee networks.

use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use log::debug;
use macaddr::MacAddr8;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot::channel;
use tokio::task::{JoinError, JoinHandle};

pub use self::builder::Builder;
pub use self::message::Message;
use self::response_receiver::ResponseReceiver;
pub use self::scans::Scans;
use crate::ember::aps;
use crate::ember::message::Destination;
use crate::error::Status;
use crate::ezsp::network::scan;
use crate::parameters::networking::handler::{EnergyScanResult, NetworkFound};
use crate::types::ByteSizedVec;
use crate::{Callback, Error, Messaging, Networking, ember};

mod builder;
mod message;
mod response_receiver;
mod scans;

/// Network manager for Zigbee networks.
#[derive(Debug)]
pub struct Ncp<T> {
    transport: T,
    profile: u16,
    aps_options: aps::Options,
    message_tag: u8,
    aps_seq: u8,
    transaction_seq: u8,
    event_handler_proxy: Sender<Message>,
    event_handler_handle: JoinHandle<()>,
}

impl<T> Ncp<T> {
    /// Creates a new `NetworkManager` with the given transport.
    #[must_use]
    pub(crate) const fn new(
        transport: T,
        profile: u16,
        aps_options: aps::Options,
        event_handler_proxy: Sender<Message>,
        event_handler_handle: JoinHandle<()>,
    ) -> Self {
        Self {
            transport,
            profile,
            aps_options,
            message_tag: 0,
            aps_seq: 0,
            transaction_seq: 0,
            event_handler_proxy,
            event_handler_handle,
        }
    }

    /// Creates a new `Builder` for constructing a `NetworkManager`.
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

    pub const fn next_transaction_seq(&mut self) -> u8 {
        let seq = self.transaction_seq;
        self.transaction_seq = self.transaction_seq.wrapping_add(1);
        seq
    }

    /// Creates a new APS frame with the given parameters.
    fn next_aps_frame(
        &mut self,
        profile_id: Option<u16>,
        cluster_id: u16,
        source_endpoint: u8,
        destination_endpoint: u8,
        group_id: u16,
    ) -> aps::Frame {
        aps::Frame::new(
            profile_id.unwrap_or(self.profile),
            cluster_id,
            source_endpoint,
            destination_endpoint,
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
    pub async fn unicast(
        &mut self,
        short_id: u16,
        profile_id: Option<u16>,
        cluster_id: u16,
        source_endpoint: u8,
        destination_endpoint: u8,
        message: ByteSizedVec<u8>,
    ) -> Result<ResponseReceiver, Error> {
        let tag = self.next_message_tag();
        let aps_frame = self.next_aps_frame(
            profile_id,
            cluster_id,
            source_endpoint,
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

        Ok(ResponseReceiver::new(rx, seq))
    }

    pub async fn multicast(
        &mut self,
        group_id: u16,
        hops: u8,
        radius: u8,
        profile_id: Option<u16>,
        cluster_id: u16,
        source_endpoint: u8,
        message: ByteSizedVec<u8>,
    ) -> Result<ResponseReceiver, Error> {
        let tag = self.next_message_tag();
        let aps_frame =
            self.next_aps_frame(profile_id, cluster_id, source_endpoint, 0xFF, group_id);

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

        Ok(ResponseReceiver::new(rx, seq))
    }

    pub async fn broadcast(
        &mut self,
        short_id: u16,
        radius: u8,
        profile_id: Option<u16>,
        cluster_id: u16,
        source_endpoint: u8,
        message: ByteSizedVec<u8>,
    ) -> Result<u8, Error> {
        let tag = self.next_message_tag();
        let aps_frame = self.next_aps_frame(profile_id, cluster_id, source_endpoint, 0xFF, 0x0000);
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
    /// Creates a new `Builder` for constructing a `NetworkManager`.
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
