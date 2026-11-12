use std::collections::VecDeque;
use std::mem;

use log::debug;
use tokio::sync::oneshot::Sender;

use self::scan::Scan;
use crate::Error;
use crate::ezsp::network::scan::Type;
use crate::parameters::networking::handler::{EnergyScanResult, NetworkFound};

mod scan;

/// Aggregates scan callbacks until the matching `scanComplete` callback arrives.
///
/// EZSP reports active network scans and energy scans as a stream of callbacks,
/// followed by `scanComplete`. `Scans` keeps accepted scans in order and buffers
/// results for the oldest scan. The event handler resolves its result channel
/// with either the collected values or the completion error, then clears the
/// buffers. Canceled accepted scans retain their place until completion.
#[derive(Debug, Default)]
pub struct Scans {
    queue: VecDeque<PendingScan>,
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
        self.queue.push_back(PendingScan::Legacy(scan));
    }

    /// Adds an energy scan result to the current channel buffer.
    pub fn add_channel(&mut self, scanned: EnergyScanResult) {
        if self
            .queue
            .front()
            .is_some_and(|scan| scan.kind() == Type::EnergyScan)
        {
            self.channels.push(scanned);
        }
    }

    /// Adds an active scan result to the current network buffer.
    pub fn add_network(&mut self, found: NetworkFound) {
        if self
            .queue
            .front()
            .is_some_and(|scan| scan.kind() == Type::ActiveScan)
        {
            self.networks.push(found);
        }
    }

    /// Completes the oldest pending scan successfully with its buffered results.
    pub fn pop(&mut self) {
        self.complete(Ok(()));
    }

    /// Registers an accepted scan, retaining even canceled callers until completion.
    pub(super) fn register(&mut self, scan: PendingScan) {
        self.queue.push_back(scan);
    }

    /// Completes one scan and clears its buffers on both success and failure.
    pub(super) fn complete(&mut self, status: Result<(), Error>) {
        let channels = mem::take(&mut self.channels);
        let networks = mem::take(&mut self.networks);
        if let Some(scan) = self.queue.pop_front() {
            scan.complete(status, channels, networks);
        }
    }
}

/// Response channel for a scan whose command has been accepted by the NCP.
#[derive(Debug)]
pub(super) enum PendingScan {
    /// Compatibility path for callers that issue their own scan command.
    Legacy(Scan),
    /// An energy scan with command and completion error reporting.
    Channel(Sender<Result<Vec<EnergyScanResult>, Error>>),
    /// An active scan with command and completion error reporting.
    Network(Sender<Result<Vec<NetworkFound>, Error>>),
}

impl PendingScan {
    /// Returns the corresponding EZSP scan type.
    pub(super) const fn kind(&self) -> Type {
        match self {
            Self::Legacy(Scan::Channel(_)) | Self::Channel(_) => Type::EnergyScan,
            Self::Legacy(Scan::Network(_)) | Self::Network(_) => Type::ActiveScan,
        }
    }

    /// Checks cancellation before issuing a new command.
    pub(super) fn is_closed(&self) -> bool {
        match self {
            Self::Legacy(Scan::Channel(sender)) => sender.is_closed(),
            Self::Legacy(Scan::Network(sender)) => sender.is_closed(),
            Self::Channel(sender) => sender.is_closed(),
            Self::Network(sender) => sender.is_closed(),
        }
    }

    /// Delivers results or the original error; legacy receivers close on failure.
    pub(super) fn complete(
        self,
        status: Result<(), Error>,
        channels: Vec<EnergyScanResult>,
        networks: Vec<NetworkFound>,
    ) {
        let delivered = match self {
            Self::Legacy(Scan::Channel(sender)) => match status {
                Ok(()) => sender.send(channels).is_ok(),
                Err(error) => {
                    debug!("Legacy energy scan failed: {error}");
                    return;
                }
            },
            Self::Legacy(Scan::Network(sender)) => match status {
                Ok(()) => sender.send(networks).is_ok(),
                Err(error) => {
                    debug!("Legacy network scan failed: {error}");
                    return;
                }
            },
            Self::Channel(sender) => sender.send(status.map(|()| channels)).is_ok(),
            Self::Network(sender) => sender.send(status.map(|()| networks)).is_ok(),
        };
        if !delivered {
            debug!("Scan result receiver closed");
        }
    }
}

#[cfg(test)]
mod tests {
    use le_stream::FromLeStream;
    use tokio::sync::oneshot;

    use super::{PendingScan, Scans};
    use crate::Error;
    use crate::parameters::networking::handler::EnergyScanResult;

    const RESULT_BYTES: [u8; 2] = [11, 0xD8];

    fn energy_result() -> EnergyScanResult {
        EnergyScanResult::from_le_stream(RESULT_BYTES.into_iter()).unwrap()
    }

    #[test]
    fn legacy_registration_still_receives_successful_results() {
        let mut scans = Scans::new();
        let (sender, result) = oneshot::channel::<Vec<EnergyScanResult>>();
        scans.push(sender.into());
        scans.add_channel(energy_result());
        scans.pop();
        assert_eq!(result.blocking_recv().unwrap(), vec![energy_result()]);
    }

    #[test]
    fn legacy_failure_closes_the_channel_and_clears_buffers() {
        let mut scans = Scans::new();
        let (sender, result) = oneshot::channel::<Vec<EnergyScanResult>>();
        scans.push(sender.into());
        scans.add_channel(energy_result());
        scans.complete(Err(Error::NotConfigured));
        assert!(result.blocking_recv().is_err());
        assert_eq!(scans.channels, []);
        assert!(scans.queue.is_empty());
    }

    #[test]
    fn ignores_unsolicited_results_before_a_scan() {
        let mut scans = Scans::new();
        scans.add_channel(energy_result());
        let (sender, result) = oneshot::channel();
        scans.register(PendingScan::Channel(sender));
        scans.complete(Ok(()));
        assert_eq!(result.blocking_recv().unwrap().unwrap(), []);
    }
}
