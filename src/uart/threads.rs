use std::sync::Arc;

use ashv2::{Actor, Proxy, TTYPort};
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
pub struct Threads {
    ashv2_transmitter: tokio::task::JoinHandle<()>,
    ashv2_receiver: tokio::task::JoinHandle<()>,
    splitter: tokio::task::JoinHandle<()>,
}

impl Threads {
    /// Spawn the threads for the UART communication.
    pub fn spawn(
        serial_port: TTYPort,
        callbacks_tx: Sender<Callback>,
        state: Arc<NpRwLock<State>>,
        channel_size: usize,
    ) -> (Proxy, Receiver<Result<Parameters, Error>>, Self) {
        // `ASHv2` actor
        let (actor, ash_proxy, ash_rx) =
            Actor::new(serial_port, 64, 64).expect("Actor creation should succeed.");

        // Frame splitter
        let (response_tx, response_rx) = channel(channel_size);
        let splitter = spawn(
            Splitter::new(
                Decoder::new(ash_rx, state.clone()),
                response_tx,
                callbacks_tx,
                state,
            )
            .run(),
        );
        let (ashv2_transmitter, ashv2_receiver) = actor.spawn();

        let instance = Self {
            ashv2_transmitter,
            ashv2_receiver,
            splitter,
        };
        (ash_proxy, response_rx, instance)
    }
}
