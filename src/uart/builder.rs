use ashv2::{SerialPort, start};
use tokio::sync::mpsc::channel;

use crate::uart::futures::Futures;
use crate::uart::{AshRx, AshTx};
use crate::{Builder, Connectable, Receiver, Transmitter};

const DEFAULT_CHANNEL_SIZE: usize = 128;

impl Builder {
    /// Creates an actor-backed EZSP builder and `ASHv2` actor futures using the
    /// default channel capacity.
    ///
    /// The returned `Futures` contains five futures. The caller must spawn
    /// them as tasks in this order: `ASHv2` serial worker, `ASHv2` transmitter,
    /// `ASHv2` receiver, EZSP transmitter, and EZSP receiver. Spawn all five
    /// before awaiting [`Builder::start`](crate::Builder::start); otherwise a
    /// higher layer can wait indefinitely for a lower layer that is not being
    /// driven.
    #[must_use]
    pub fn ashv2<T>(
        serial_port: T,
    ) -> (
        Self,
        Futures<
            impl Future<Output = T> + Send + 'static,
            impl Future<Output = ()> + Send + 'static,
            impl Future<Output = ()> + Send + 'static,
            impl Future<Output = ()> + Send + 'static,
            impl Future<Output = ()> + Send + 'static,
        >,
    )
    where
        T: SerialPort + Sync + 'static,
    {
        Self::ashv2_with_buffers(serial_port, DEFAULT_CHANNEL_SIZE)
    }

    /// Creates an actor-backed EZSP builder and `ASHv2` actor futures with an
    /// explicit channel capacity.
    ///
    /// `channel_size` bounds the channels carrying `ASHv2` DATA payloads,
    /// `ASHv2` actor messages, EZSP actor messages, and EZSP callbacks. The
    /// returned `Futures` contains five futures. The caller must spawn them as
    /// tasks in this order: `ASHv2` serial worker, `ASHv2` transmitter, `ASHv2`
    /// receiver, EZSP transmitter, and EZSP receiver. Spawn all five before awaiting
    /// [`Builder::start`](crate::Builder::start); otherwise a higher layer can
    /// wait indefinitely for a lower layer that is not being driven.
    ///
    /// # Panics
    ///
    /// Panics if `channel_size` is zero.
    #[must_use]
    pub fn ashv2_with_buffers<T>(
        serial_port: T,
        channel_size: usize,
    ) -> (
        Self,
        Futures<
            impl Future<Output = T> + Send + 'static,
            impl Future<Output = ()> + Send + 'static,
            impl Future<Output = ()> + Send + 'static,
            impl Future<Output = ()> + Send + 'static,
            impl Future<Output = ()> + Send + 'static,
        >,
    )
    where
        T: SerialPort + Sync + 'static,
    {
        let (ash_tx, ash_rx) = channel(channel_size);
        let (ashv2, ash_futures) = start(serial_port, ash_tx);
        let (msg_tx, msg_rx) = channel(channel_size);
        let (cb_tx, cb_rx) = channel(channel_size);
        let ezsp_tx = Transmitter::new(AshTx::new(ashv2), msg_rx).run();
        let ezsp_rx = Receiver::new(AshRx::new(ash_rx), cb_tx, msg_tx.clone()).run();
        let disconnected = Connectable {
            handle: msg_tx,
            callbacks: cb_rx,
        };
        (
            Self::new(disconnected),
            Futures {
                ash_futures,
                ezsp_tx,
                ezsp_rx,
            },
        )
    }
}
