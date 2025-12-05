//! Network manager for Zigbee networks.

use std::collections::BTreeMap;
use std::io;
use std::time::Duration;

use log::info;
use macaddr::MacAddr8;
use tokio_mpmc::Receiver;
use zigbee::Endpoint;
use zigbee_nwk::Nlme;

use crate::ember::aps;
use crate::ember::message::Destination;
use crate::error::Status;
use crate::types::ByteSizedVec;
use crate::zigbee::network_manager::builder::Builder;
use crate::{Callback, Configuration, Error, Messaging, Networking, Security, Utilities, ember};

mod builder;
mod handle_incoming_message;
mod handle_message_sent;
mod stack_status;

/// Network manager for Zigbee networks.
pub struct NetworkManager<T> {
    transport: T,
    aps_options: aps::Options,
    profile_id: u16,
    message_tag: u8,
    aps_seq: u8,
    transaction_seq: u8,
}

impl<T> NetworkManager<T> {
    /// Creates a new `NetworkManager` with the given transport.
    #[must_use]
    pub(crate) const fn new(transport: T, profile_id: u16, aps_options: aps::Options) -> Self {
        Self {
            transport,
            aps_options,
            profile_id,
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

    const fn next_aps_seq(&mut self) -> u8 {
        let seq = self.aps_seq;
        self.aps_seq = self.aps_seq.wrapping_add(1);
        seq
    }

    const fn next_transaction_seq(&mut self) -> u8 {
        let seq = self.transaction_seq;
        self.transaction_seq = self.transaction_seq.wrapping_add(1);
        seq
    }
}

impl<T> Nlme for NetworkManager<T>
where
    T: Configuration + Security + Messaging + Networking + Utilities,
{
    fn get_transaction_seq(&mut self) -> u8 {
        self.next_transaction_seq()
    }

    async fn get_pan_id(&mut self) -> Result<u16, zigbee_nwk::Error> {
        let (_, parameters) = self.transport.get_network_parameters().await?;
        Ok(parameters.pan_id())
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

    async fn unicast(
        &mut self,
        pan_id: u16,
        endpoint: Endpoint,
        cluster_id: u16,
        payload: Vec<u8>,
    ) -> Result<(), zigbee_nwk::Error> {
        let tag = self.next_message_tag();
        let mut seq = self.next_aps_seq();
        seq = self
            .transport
            .send_unicast(
                Destination::Direct(pan_id),
                aps::Frame::new(
                    self.profile_id,
                    cluster_id,
                    0x01,
                    endpoint.into(),
                    self.aps_options,
                    0x00, // TODO: Pass this in or get from context.
                    seq,
                ),
                tag,
                ByteSizedVec::from_slice(&payload)
                    .map_err(io::Error::other)
                    .map_err(Error::from)?,
            )
            .await?;
        self.aps_seq = seq.wrapping_add(1);
        Ok(())
    }
}
