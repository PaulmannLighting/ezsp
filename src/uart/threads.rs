use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;

use ashv2::{Payload, Transceiver};
use serialport::SerialPort;
use tokio::spawn;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use super::decoder::Decoder;
use super::splitter::Splitter;
use crate::error::Error;
use crate::frame::Parameters;
use crate::handler::Handler;
use crate::uart::state::State;
use crate::util::NpRwLock;

/// Threads and async tasks for the UART communication.
#[derive(Debug)]
pub struct Threads {
    running: Arc<AtomicBool>,
    transceiver: Option<std::thread::JoinHandle<()>>,
    splitter: tokio::task::JoinHandle<()>,
    handler: tokio::task::JoinHandle<()>,
}

impl Threads {
    /// Spawn the threads for the UART communication.
    pub fn spawn<S, H>(
        serial_port: S,
        handler: H,
        state: Arc<NpRwLock<State>>,
        channel_size: usize,
    ) -> (Sender<Payload>, Receiver<Result<Parameters, Error>>, Self)
    where
        S: SerialPort + 'static,
        H: Handler,
    {
        let running = Arc::new(AtomicBool::new(true));

        // `ASHv2` transceiver
        let (frames_out, frames_in, transceiver) =
            Transceiver::spawn(serial_port, running.clone(), channel_size);

        // Callback handler
        let (callbacks_tx, callbacks_rx) = channel(channel_size);
        let handler = spawn(handler.run(callbacks_rx));

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
            handler,
        };
        (frames_out, response_rx, instance)
    }
}

/// Tear down the threads for the UART communication.
impl Drop for Threads {
    fn drop(&mut self) {
        // First stop the ASHv2 transceiver...
        self.running.store(false, Relaxed);

        if let Some(transceiver) = self.transceiver.take() {
            transceiver
                .join()
                .expect("Transceiver thread should be able to join. This is a bug.");
        }

        // ...then stop the frame splitter and callback handler.
        self.splitter.abort();
        self.handler.abort();
    }
}
