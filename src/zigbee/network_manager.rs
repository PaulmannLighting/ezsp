//! Network manager for Zigbee networks.

use std::collections::BTreeMap;
use std::time::Duration;

use le_stream::ToLeStream;
use log::info;
use macaddr::MacAddr8;
use tokio::sync::mpsc::Receiver;
use zigbee_nwk::aps::Command;
use zigbee_nwk::zcl::Cluster;
use zigbee_nwk::{Nlme, zcl};

pub use self::event_manager::EventManager;
use crate::ember::aps;
use crate::ember::message::Destination;
use crate::error::Status;
use crate::zigbee::network_manager::builder::Builder;
use crate::{Callback, Configuration, Error, Messaging, Networking, Security, Utilities, ember};

mod builder;
mod event_manager;

/// Network manager for Zigbee networks.
pub struct NetworkManager<T> {
    transport: T,
    event_manager: EventManager,
    aps_options: aps::Options,
    profile_id: u16,
    message_tag: u8,
    aps_seq: u8,
}

impl<T> NetworkManager<T> {
    /// Creates a new `NetworkManager` with the given transport.
    #[must_use]
    pub(crate) const fn new(
        transport: T,
        event_manager: EventManager,
        profile_id: u16,
        aps_options: aps::Options,
    ) -> Self {
        Self {
            transport,
            event_manager,
            aps_options,
            profile_id,
            message_tag: 0,
            aps_seq: 0,
        }
    }

    /// Creates a new `Builder` for constructing a `NetworkManager`.
    #[must_use]
    pub fn build(transport: T, callbacks_rx: Receiver<Callback>) -> Builder<T> {
        Builder::new(transport, callbacks_rx)
    }

    /// Returns a mutable reference to the event manager.
    #[must_use]
    pub const fn event_manager(&mut self) -> &mut EventManager {
        &mut self.event_manager
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
}

impl<T> Nlme for NetworkManager<T>
where
    T: Configuration + Security + Messaging + Networking + Utilities,
{
    type Error = Error;

    async fn allow_joins(
        &mut self,
        duration: Duration,
    ) -> Result<(), zigbee_nwk::Error<Self::Error>> {
        info!("Allowing joins for {} seconds.", duration.as_secs());
        self.transport
            .permit_joining(u8::try_from(duration.as_secs()).unwrap_or(u8::MAX).into())
            .await
            .map_err(Into::into)
    }

    async fn get_neighbors(
        &mut self,
    ) -> Result<BTreeMap<MacAddr8, Option<u16>>, zigbee_nwk::Error<Self::Error>> {
        let mut neighbors = BTreeMap::new();

        for index in 0..=u8::MAX {
            match self.transport.get_neighbor(index).await {
                Ok(neighbor) => {
                    neighbors.insert(neighbor.long_id(), Some(neighbor.short_id()));
                }
                Err(error) => match error {
                    Error::Status(Status::Ember(Ok(ember::Status::ErrFatal))) => break,
                    error => return Err(error.into()),
                },
            }
        }

        Ok(neighbors)
    }

    async fn unicast_command<P>(
        &mut self,
        destination: u16,
        frame: Command<P>,
    ) -> Result<(), zigbee_nwk::Error<Self::Error>>
    where
        P: zcl::Command + ToLeStream,
    {
        let tag = self.next_message_tag();
        let mut seq = self.next_aps_seq();
        seq = self
            .transport
            .send_unicast(
                Destination::Direct(destination),
                aps::Frame::new(
                    self.profile_id,
                    <P as Cluster>::ID,
                    0x01,
                    0x01,
                    self.aps_options,
                    0x00,
                    seq,
                ),
                tag,
                frame.into_payload().to_le_stream().collect(),
            )
            .await?;
        self.aps_seq = seq.wrapping_add(1);
        Ok(())
    }
}
