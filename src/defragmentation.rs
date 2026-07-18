//! Host-side reassembly for fragmented incoming APS messages.
//!
//! This mirrors the receive-side rules of Silicon Labs' EZSP fragmentation
//! component: a message is identified by its sender and APS sequence number,
//! fragments may arrive out of order inside the receive window, and incomplete
//! messages expire when [`Defragmenter::tick`] observes their timeout.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use const_env::env_item;

use crate::ember::NodeId;
use crate::parameters::messaging::handler::IncomingMessage;

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

/// An incoming APS message after fragment handling.
#[derive(Debug)]
pub enum Message {
    /// A non-fragmented message that can be forwarded immediately.
    Complete(IncomingMessage),

    /// A fragmented message whose payload has been reassembled.
    Defragmented {
        /// The callback that completed the fragmented message.
        incoming_message: IncomingMessage,
        /// The reassembled APS payload.
        payload: Vec<u8>,
    },

    /// A fragment was stored or deliberately ignored.
    Incomplete,
}

/// Reassembles fragmented incoming APS messages.
///
/// Construct this value during callback-handler initialization, call
/// [`Defragmenter::handle`] before processing every incoming APS callback, and
/// call [`Defragmenter::tick`] regularly. This is the Rust equivalent of
/// `ezspFragmentInit`, `ezspFragmentIncomingMessage`, and `ezspFragmentTick`.
#[derive(Debug)]
pub struct Defragmenter {
    packets: BTreeMap<PacketKey, Packet>,
    receive_buffer_length: usize,
    window_size: u8,
    timeout: Duration,
}

impl Default for Defragmenter {
    fn default() -> Self {
        Self::new(
            DEFAULT_RECEIVE_BUFFER_LENGTH,
            DEFAULT_WINDOW_SIZE,
            DEFAULT_REASSEMBLY_TIMEOUT,
        )
    }
}

impl Defragmenter {
    /// Initializes a fragment receiver with a bounded reassembly buffer.
    ///
    /// A zero or unsupported `window_size` disables fragmented-message
    /// reception, matching the EZSP host implementation's behavior.
    #[must_use]
    pub const fn new(receive_buffer_length: usize, window_size: u8, timeout: Duration) -> Self {
        Self {
            packets: BTreeMap::new(),
            receive_buffer_length,
            window_size,
            timeout,
        }
    }

    /// Handles one incoming APS callback.
    #[must_use]
    pub fn handle(&mut self, incoming_message: IncomingMessage) -> Message {
        self.handle_at(incoming_message, Instant::now())
    }

    /// Drops incomplete messages whose APS retry period has elapsed.
    pub fn tick(&mut self) {
        self.tick_at(Instant::now());
    }

    fn handle_at(&mut self, incoming_message: IncomingMessage, now: Instant) -> Message {
        let Some((fragment, expected_fragments)) = incoming_message.aps_frame().fragmentation()
        else {
            return Message::Complete(incoming_message);
        };

        if self.window_size == 0 || self.window_size > MAX_WINDOW_SIZE {
            return Message::Incomplete;
        }

        let key = PacketKey {
            sender: incoming_message.sender(),
            sequence: incoming_message.aps_frame().sequence(),
        };

        if !self.packets.contains_key(&key) {
            if self.packets.len() >= MAX_INCOMING_PACKETS || fragment >= self.window_size {
                return Message::Incomplete;
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

        completed.map_or_else(
            || Message::Incomplete,
            |payload| Message::Defragmented {
                incoming_message,
                payload,
            },
        )
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::time::Instant;

    use super::{DEFAULT_REASSEMBLY_TIMEOUT, Defragmenter, Packet, PacketKey};

    const SENDER: u16 = 0x1234;
    const SEQUENCE: u8 = 0x56;
    const WINDOW_SIZE: u8 = 2;
    const RECEIVE_BUFFER_LENGTH: usize = 8;
    const EXPECTED_FRAGMENTS: u8 = 2;
    const FIRST_FRAGMENT: u8 = 0;
    const SECOND_FRAGMENT: u8 = 1;
    const FIRST_PAYLOAD: &[u8] = b"one";
    const SECOND_PAYLOAD: &[u8] = b"two";

    #[test]
    fn expires_incomplete_packets() {
        let now = Instant::now();
        let mut defragmenter = Defragmenter::new(
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

        assert!(!Defragmenter::window_is_complete(
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
        let mut defragmenter = Defragmenter::new(
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
