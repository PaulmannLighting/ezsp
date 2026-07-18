//! Host-side reassembly for fragmented incoming APS messages.
//!
//! This mirrors the receive-side rules of Silicon Labs' EZSP fragmentation
//! component: a message is identified by its sender and APS sequence number,
//! fragments may arrive out of order inside the receive window, and incomplete
//! messages expire when [`Defragmenter::tick`] observes their timeout. Every
//! received fragment is acknowledged through EZSP `sendReply` before it is
//! stored.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use const_env::env_item;
use log::warn;

pub use self::defragmented_message::DefragmentedMessage;
use crate::ember::NodeId;
use crate::parameters::messaging::handler::IncomingMessage;
use crate::types::ByteSizedVec;
use crate::{Messaging, SharedTransport, Transport};

mod defragmented_message;

/// The number of fragmented messages accepted concurrently by the EZSP host.
///
/// Configure at compile time with `EZSP_DEFRAGMENTATION_MAX_INCOMING_PACKETS`.
#[env_item("EZSP_DEFRAGMENTATION_MAX_INCOMING_PACKETS")]
pub const MAX_INCOMING_PACKETS: usize = 1;
/// The default number of fragments accepted in one receive window.
///
/// Configure at compile time with `EZSP_DEFRAGMENTATION_DEFAULT_WINDOW_SIZE`.
#[env_item("EZSP_DEFRAGMENTATION_DEFAULT_WINDOW_SIZE")]
pub const DEFAULT_WINDOW_SIZE: u8 = 1;
/// The largest receive window supported by the APS fragment bit mask.
pub const MAX_WINDOW_SIZE: u8 = 8;
/// The largest reassembled payload accepted by the host adapter.
///
/// Configure at compile time with `EZSP_DEFRAGMENTATION_RECEIVE_BUFFER_LENGTH`.
#[env_item("EZSP_DEFRAGMENTATION_RECEIVE_BUFFER_LENGTH")]
pub const DEFAULT_RECEIVE_BUFFER_LENGTH: usize = 4_096;
/// The timeout used when the NCP's APS retry timeout is not available.
///
/// Configure at compile time with
/// `EZSP_DEFRAGMENTATION_REASSEMBLY_TIMEOUT_MILLIS`.
#[env_item("EZSP_DEFRAGMENTATION_REASSEMBLY_TIMEOUT_MILLIS")]
pub const DEFAULT_REASSEMBLY_TIMEOUT_MILLIS: u64 = 5_000;
/// The timeout used when the NCP's APS retry timeout is not available.
pub const DEFAULT_REASSEMBLY_TIMEOUT: Duration =
    Duration::from_millis(DEFAULT_REASSEMBLY_TIMEOUT_MILLIS);

/// A complete incoming message produced by a [`Defragmenter`].
pub type Defragmented = DefragmentedMessage;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct PacketKey {
    sender: NodeId,
    sequence: u8,
}

#[derive(Debug)]
struct Packet {
    fragments: BTreeMap<u8, Vec<u8>>,
    expected_fragments: Option<u8>,
    window_base: u8,
    deadline: Instant,
}

/// Reassembles fragmented incoming APS messages.
///
/// Construct this value during callback-handler initialization, call
/// [`Defragmenter::handle`] before processing every incoming APS callback, and
/// call [`Defragmenter::tick`] regularly. This is the Rust equivalent of
/// `ezspFragmentInit`, `ezspFragmentIncomingMessage`, and `ezspFragmentTick`.
#[derive(Debug)]
pub struct Defragmenter<T>
where
    T: Transport,
{
    transport: SharedTransport<T>,
    packets: BTreeMap<PacketKey, Packet>,
    receive_buffer_length: usize,
    window_size: u8,
    timeout: Duration,
}

impl<T> Defragmenter<T>
where
    T: Transport,
{
    /// Creates a defragmenter using the default receive limits.
    #[must_use]
    pub fn new(transport: impl Into<SharedTransport<T>>) -> Self {
        Self::with_configuration(
            transport,
            DEFAULT_RECEIVE_BUFFER_LENGTH,
            DEFAULT_WINDOW_SIZE,
            DEFAULT_REASSEMBLY_TIMEOUT,
        )
    }

    /// Creates a defragmenter with explicit receive limits.
    ///
    /// A zero or unsupported `window_size` disables fragmented-message
    /// reception, matching the EZSP host implementation's behavior.
    #[must_use]
    pub fn with_configuration(
        transport: impl Into<SharedTransport<T>>,
        receive_buffer_length: usize,
        window_size: u8,
        timeout: Duration,
    ) -> Self {
        Self {
            transport: transport.into(),
            packets: BTreeMap::new(),
            receive_buffer_length,
            window_size,
            timeout,
        }
    }

    /// Drops incomplete messages whose APS retry period has elapsed.
    pub fn tick(&mut self) {
        self.tick_at(Instant::now());
    }

    fn handle_fragment_at(
        &mut self,
        incoming_message: &IncomingMessage,
        fragment: u8,
        expected_fragments: Option<u8>,
        now: Instant,
    ) -> Option<Defragmented> {
        if self.window_size == 0 || self.window_size > MAX_WINDOW_SIZE {
            return None;
        }

        let key = PacketKey {
            sender: incoming_message.sender(),
            sequence: incoming_message.aps_frame().sequence(),
        };

        if !self.packets.contains_key(&key) {
            if self.packets.len() >= MAX_INCOMING_PACKETS || fragment >= self.window_size {
                return None;
            }

            self.packets.insert(
                key,
                Packet {
                    fragments: BTreeMap::new(),
                    expected_fragments: None,
                    window_base: 0,
                    deadline: now + self.timeout,
                },
            );
        }

        let payload = incoming_message.message();
        let completed = self.store_fragment(key, fragment, expected_fragments, payload, now);

        completed.map(|payload| {
            DefragmentedMessage::from_incoming_message(incoming_message, payload.into())
        })
    }

    fn store_fragment(
        &mut self,
        key: PacketKey,
        fragment: u8,
        expected_fragments: Option<u8>,
        payload: &[u8],
        now: Instant,
    ) -> Option<Vec<u8>> {
        let timeout = self.timeout;
        let window_size = self.window_size;
        let receive_buffer_length = self.receive_buffer_length;
        let (discard, complete) = {
            let packet = self.packets.get_mut(&key)?;

            if fragment < packet.window_base
                || fragment >= packet.window_base.saturating_add(self.window_size)
            {
                return None;
            }

            if let Some(expected_fragments) = expected_fragments {
                if expected_fragments == 0
                    || packet
                        .expected_fragments
                        .is_some_and(|expected| expected != expected_fragments)
                {
                    (true, false)
                } else {
                    packet.expected_fragments = Some(expected_fragments);
                    Self::store_and_check(
                        packet,
                        fragment,
                        payload,
                        now,
                        timeout,
                        window_size,
                        receive_buffer_length,
                    )
                }
            } else {
                Self::store_and_check(
                    packet,
                    fragment,
                    payload,
                    now,
                    timeout,
                    window_size,
                    receive_buffer_length,
                )
            }
        };

        if discard {
            self.packets.remove(&key);
            return None;
        }

        if !complete {
            return None;
        }

        let packet = self.packets.remove(&key)?;
        Some(packet.fragments.into_values().flatten().collect())
    }

    fn store_and_check(
        packet: &mut Packet,
        fragment: u8,
        payload: &[u8],
        now: Instant,
        timeout: Duration,
        window_size: u8,
        receive_buffer_length: usize,
    ) -> (bool, bool) {
        packet.deadline = now + timeout;
        packet
            .fragments
            .entry(fragment)
            .or_insert_with(|| payload.to_vec());

        let Some(expected_fragments) = packet.expected_fragments else {
            return (false, false);
        };
        if !Self::window_is_complete(packet, expected_fragments, window_size) {
            return (false, false);
        }

        while Self::window_is_complete(packet, expected_fragments, window_size)
            && packet.window_base.saturating_add(window_size) < expected_fragments
        {
            packet.window_base = packet.window_base.saturating_add(window_size);
        }

        if packet.fragments.len() != usize::from(expected_fragments) {
            return (false, false);
        }

        let message_length: usize = packet.fragments.values().map(Vec::len).sum();
        (message_length > receive_buffer_length, true)
    }

    fn window_is_complete(packet: &Packet, expected_fragments: u8, window_size: u8) -> bool {
        let window_end = packet
            .window_base
            .saturating_add(window_size)
            .min(expected_fragments);
        (packet.window_base..window_end).all(|fragment| packet.fragments.contains_key(&fragment))
    }

    fn tick_at(&mut self, now: Instant) {
        self.packets.retain(|_, packet| packet.deadline > now);
    }
}

impl<T> Defragmenter<T>
where
    T: Messaging + Transport,
{
    /// Handles one incoming APS callback.
    ///
    /// Complete messages are returned immediately. Fragmented messages return
    /// `None` until every fragment has arrived. A fragment is acknowledged with
    /// an empty EZSP `sendReply`; if that command fails, the fragment is not
    /// stored and this method returns `None` so that a retransmission can be
    /// handled later.
    #[must_use]
    pub async fn handle(&mut self, incoming_message: IncomingMessage) -> Option<Defragmented> {
        let Some((fragment, expected_fragments)) = incoming_message.aps_frame().fragmentation()
        else {
            return Some(incoming_message.into());
        };

        let reply = {
            let mut transport = self.transport.lock().await;
            transport
                .send_reply(
                    incoming_message.sender(),
                    incoming_message.aps_frame().clone(),
                    ByteSizedVec::new(),
                )
                .await
        };

        if let Err(error) = reply {
            warn!("Failed to acknowledge incoming APS fragment: {error}");
            return None;
        }

        let message = self.handle_fragment_at(
            &incoming_message,
            fragment,
            expected_fragments,
            Instant::now(),
        );
        drop(incoming_message);
        message
    }
}

#[cfg(test)]
mod tests {
    use core::future::Future;
    use std::collections::BTreeMap;
    use std::num::NonZero;
    use std::time::Instant;

    use le_stream::ToLeStream;

    use super::{DEFAULT_REASSEMBLY_TIMEOUT, Defragmenter, Packet, PacketKey};
    use crate::frame::Parameter;
    use crate::parameters::configuration::version;
    use crate::{Connection, Error, Parameters, Transport};

    const SENDER: u16 = 0x1234;
    const SEQUENCE: u8 = 0x56;
    const WINDOW_SIZE: u8 = 2;
    const RECEIVE_BUFFER_LENGTH: usize = 8;
    const EXPECTED_FRAGMENTS: u8 = 2;
    const FIRST_FRAGMENT: u8 = 0;
    const SECOND_FRAGMENT: u8 = 1;
    const FIRST_PAYLOAD: &[u8] = b"one";
    const SECOND_PAYLOAD: &[u8] = b"two";

    #[derive(Debug)]
    struct MockTransport;

    #[expect(
        clippy::manual_async_fn,
        reason = "trait implementations must return a Send future explicitly"
    )]
    impl Transport for MockTransport {
        fn connect(&mut self) -> impl Future<Output = Result<version::Response, Error>> + Send {
            async { Err(Error::NotConfigured) }
        }

        fn state(&self) -> Connection {
            Connection::Disconnected
        }

        fn negotiated_version(&self) -> Option<NonZero<u8>> {
            None
        }

        fn send<T>(&mut self, _command: T) -> impl Future<Output = Result<u16, Error>> + Send
        where
            T: Parameter + ToLeStream,
        {
            async { Err(Error::NotConfigured) }
        }

        fn receive<T>(&mut self) -> impl Future<Output = Result<T, Error>> + Send
        where
            T: TryFrom<Parameters> + Send,
            <T as TryFrom<Parameters>>::Error: Into<Parameters> + Send,
        {
            async { Err(Error::NotConfigured) }
        }
    }

    #[test]
    fn expires_incomplete_packets() {
        let now = Instant::now();
        let mut defragmenter = Defragmenter::with_configuration(
            MockTransport,
            RECEIVE_BUFFER_LENGTH,
            WINDOW_SIZE,
            DEFAULT_REASSEMBLY_TIMEOUT,
        );
        defragmenter.packets.insert(
            PacketKey {
                sender: SENDER,
                sequence: SEQUENCE,
            },
            Packet {
                fragments: BTreeMap::new(),
                expected_fragments: None,
                window_base: 0,
                deadline: now,
            },
        );

        defragmenter.tick_at(now);

        assert!(defragmenter.packets.is_empty());
    }

    #[test]
    fn incomplete_window_is_not_complete() {
        let packet = Packet {
            fragments: BTreeMap::new(),
            expected_fragments: Some(EXPECTED_FRAGMENTS),
            window_base: 0,
            deadline: Instant::now() + DEFAULT_REASSEMBLY_TIMEOUT,
        };

        assert!(!Defragmenter::<MockTransport>::window_is_complete(
            &packet,
            EXPECTED_FRAGMENTS,
            WINDOW_SIZE
        ));
    }

    #[test]
    fn reassembles_out_of_order_fragments() {
        let now = Instant::now();
        let key = PacketKey {
            sender: SENDER,
            sequence: SEQUENCE,
        };
        let mut defragmenter = Defragmenter::with_configuration(
            MockTransport,
            RECEIVE_BUFFER_LENGTH,
            WINDOW_SIZE,
            DEFAULT_REASSEMBLY_TIMEOUT,
        );
        defragmenter.packets.insert(
            key,
            Packet {
                fragments: BTreeMap::new(),
                expected_fragments: None,
                window_base: 0,
                deadline: now + DEFAULT_REASSEMBLY_TIMEOUT,
            },
        );

        assert!(
            defragmenter
                .store_fragment(key, SECOND_FRAGMENT, None, SECOND_PAYLOAD, now)
                .is_none()
        );
        let payload = defragmenter
            .store_fragment(
                key,
                FIRST_FRAGMENT,
                Some(EXPECTED_FRAGMENTS),
                FIRST_PAYLOAD,
                now,
            )
            .expect("all fragments are present");

        assert_eq!(payload, [FIRST_PAYLOAD, SECOND_PAYLOAD].concat());
    }
}
