use std::collections::VecDeque;

use log::error;

use self::scan::Scan;
use crate::parameters::networking::handler::{EnergyScanResult, NetworkFound};

mod scan;

/// Struct to manage scans and their respective results.
#[derive(Debug, Default)]
pub struct Scans {
    queue: VecDeque<Scan>,
    channels: Vec<EnergyScanResult>,
    networks: Vec<NetworkFound>,
}

impl Scans {
    /// Add a new scan onto the queue.
    pub fn push(&mut self, scan: Scan) {
        self.queue.push_back(scan);
    }

    /// Add a channel to the channels buffer.
    pub fn add_channel(&mut self, scanned: EnergyScanResult) {
        self.channels.push(scanned);
    }

    /// Add a network to the networks buffer.
    pub fn add_network(&mut self, found: NetworkFound) {
        self.networks.push(found);
    }

    /// Conclude the last scan.
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
