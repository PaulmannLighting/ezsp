use ashv2::{Futures, SerialPort, start};
use tokio::spawn;
use tokio::sync::mpsc::channel;

use crate::uart::{Receiver, Transmitter};
use crate::{Builder, Transceiver};

const DEFAULT_CHANNEL_SIZE: usize = 128;

impl Builder<Transmitter, Receiver> {
    /// Creates an actor-backed EZSP builder using the default `ASHv2` channel size.
    ///
    /// The `ASHv2` serial worker, transmitter, and receiver tasks are spawned
    /// immediately. Call [`Builder::start`](crate::Builder::start) to spawn the
    /// EZSP actors and initialize the NCP.
    #[must_use]
    pub fn ashv2<T>(serial_port: T) -> Self
    where
        T: SerialPort + Sync + 'static,
    {
        Self::ashv2_with_buffers(serial_port, DEFAULT_CHANNEL_SIZE)
    }

    /// Creates an actor-backed EZSP builder with an explicit `ASHv2` channel size.
    ///
    /// `channel_size` bounds the channel carrying received `ASHv2` DATA payloads.
    /// The `ASHv2` worker tasks are spawned immediately; EZSP actor tasks start
    /// when [`Builder::start`](crate::Builder::start) is called.
    #[must_use]
    pub fn ashv2_with_buffers<T>(serial_port: T, channel_size: usize) -> Self
    where
        T: SerialPort + Sync + 'static,
    {
        let (ash_tx, ash_rx) = channel(channel_size);
        let (
            ashv2,
            Futures {
                serial_worker,
                transmitter,
                receiver,
            },
        ) = start(serial_port, ash_tx);
        spawn(serial_worker);
        spawn(transmitter);
        spawn(receiver);
        let transmitter = Transmitter::new(ashv2);
        let receiver = Receiver::new(ash_rx);
        let transceiver = Transceiver::new(transmitter, receiver);
        Self::new(transceiver)
    }
}
