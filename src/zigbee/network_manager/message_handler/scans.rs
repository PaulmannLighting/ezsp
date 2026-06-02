use std::collections::VecDeque;

use log::error;
use zigbee_hw::{FoundNetwork, ScannedChannel};

use self::scan::Scan;

mod scan;

/// Struct to manage scans and their respective results.
#[derive(Debug)]
pub struct Scans {
    scans: VecDeque<Scan>,
    channels: Vec<ScannedChannel>,
    networks: Vec<FoundNetwork>,
}

impl Scans {
    /// Create a new `Scans` instance.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            scans: VecDeque::new(),
            channels: Vec::new(),
            networks: Vec::new(),
        }
    }

    /// Add a new scan onto the queue.
    pub fn push(&mut self, scan: Scan) {
        self.scans.push_back(scan);
    }

    /// Add a channel to the channels buffer.
    pub fn add_channel(&mut self, scanned: ScannedChannel) {
        self.channels.push(scanned);
    }

    /// Add a network to the networks buffer.
    pub fn add_network(&mut self, found: FoundNetwork) {
        self.networks.push(found);
    }

    /// Conclude the last scan.
    pub async fn pop(&mut self) {
        if let Some(scan) = self.scans.pop_front() {
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
