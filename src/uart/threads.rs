use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;

use ashv2::{Payload, SerialPort, Transceiver};
use log::error;
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, channel};

use super::decoder::Decoder;
use super::splitter::Splitter;
use crate::Callback;
use crate::error::Error;
use crate::frame::Parameters;
use crate::uart::np_rw_lock::NpRwLock;
use crate::uart::state::State;

/// Threads and async tasks for the UART communication.
#[derive(Debug)]
pub struct Threads<T> {
    running: Arc<AtomicBool>,
    transceiver: Option<std::thread::JoinHandle<T>>,
    splitter: tokio::task::JoinHandle<()>,
}

impl<T> Threads<T> {
    /// Spawn the threads for the UART communication.
    pub fn spawn(
        serial_port: T,
        callbacks_tx: tokio_mpmc::Sender<Callback>,
        state: Arc<NpRwLock<State>>,
        channel_size: usize,
    ) -> (Sender<Payload>, Receiver<Result<Parameters, Error>>, Self)
    where
        T: SerialPort + 'static,
    {
        let running = Arc::new(AtomicBool::new(true));

        // `ASHv2` transceiver
        let (frames_out, frames_in, transceiver) =
            Transceiver::spawn(serial_port, running.clone(), channel_size);

        // Frame splitter
        let (response_tx, response_rx) = channel(channel_size);
        let splitter = spawn(
            Splitter::new(
                Decoder::new(frames_in, state.clone()),
                response_tx,
                callbacks_tx,
                state,
            )
            .run(),
        );

        let instance = Self {
            running,
            transceiver: Some(transceiver),
            splitter,
        };
        (frames_out, response_rx, instance)
    }

    /// Terminate the threads and return the serial port.
    pub fn terminate(mut self) -> T {
        self.try_terminate()
            .expect("Transceiver thread should be present and be able to join. This is a bug.")
    }

    fn try_terminate(&mut self) -> Option<T> {
        // First stop the ASHv2 transceiver...
        self.running.store(false, Relaxed);

        // ...then stop the transceiver thread and return the serial port.
        let serial_port = self.transceiver.take().and_then(|transceiver| {
            transceiver
                .join()
                .inspect_err(|_| error!("Failed to join transceiver thread."))
                .ok()
        });

        // Finally stop the frame splitter and callback handler.
        self.splitter.abort();
        serial_port
    }
}

/// Tear down the threads for the UART communication.
impl<T> Drop for Threads<T> {
    fn drop(&mut self) {
        self.try_terminate();
    }
}
