use std::collections::VecDeque;

use log::error;

use self::scan::Scan;
use crate::parameters::networking::handler::{EnergyScanResult, NetworkFound};

mod scan;

/// Aggregates scan callbacks until the matching `scanComplete` callback arrives.
///
/// EZSP reports active network scans and energy scans as a stream of callbacks,
/// followed by `scanComplete`. `Scans` keeps the registered scan queue and
/// result buffers so the event handler can resolve the corresponding one-shot
/// request.
#[derive(Debug, Default)]
pub struct Scans {
    queue: VecDeque<Scan>,
    channels: Vec<EnergyScanResult>,
    networks: Vec<NetworkFound>,
}

impl Scans {
    /// Creates an empty scan callback aggregator.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            channels: Vec::new(),
            networks: Vec::new(),
        }
    }

    /// Adds a new pending scan to the queue.
    pub fn push(&mut self, scan: Scan) {
        self.queue.push_back(scan);
    }

    /// Adds an energy scan result to the current channel buffer.
    pub fn add_channel(&mut self, scanned: EnergyScanResult) {
        self.channels.push(scanned);
    }

    /// Adds an active scan result to the current network buffer.
    pub fn add_network(&mut self, found: NetworkFound) {
        self.networks.push(found);
    }

    /// Completes the oldest pending scan and sends the buffered results.
    pub fn pop(&mut self) {
        if let Some(scan) = self.queue.pop_front() {
            match scan {
                Scan::Channel(sender) => sender
                    .send(self.channels.drain(..).collect())
                    .unwrap_or_else(|error| {
                        error!("Failed to send channel scan results: {error:?}");
                    }),

                Scan::Network(sender) => {
                    sender
                        .send(self.networks.drain(..).collect())
                        .unwrap_or_else(|error| {
                            error!("Failed to send network scan results: {error:?}");
                        });
                }
            }
        }
    }
}
