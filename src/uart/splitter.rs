use std::sync::Arc;

use log::trace;
use tokio::sync::mpsc::Sender;

use super::decoder::Decoder;
use super::np_rw_lock::NpRwLock;
use super::state::State;
use crate::error::Error;
use crate::frame::{Callback, Frame, Parameters};

/// Split incoming `EZSP` frames into responses and asynchronous callbacks.
#[derive(Debug)]
pub struct Splitter {
    incoming: Decoder,
    responses: Sender<Result<Parameters, Error>>,
    callbacks: Sender<Callback>,
    state: Arc<NpRwLock<State>>,
}

impl Splitter {
    /// Create a new `Splitter`.
    ///
    /// Sets the incoming frames decoder as a source for frames
    /// and the responses and callbacks sinks as destinations.
    #[must_use]
    pub const fn new(
        incoming: Decoder,
        responses: Sender<Result<Parameters, Error>>,
        callbacks: Sender<Callback>,
        state: Arc<NpRwLock<State>>,
    ) -> Self {
        Self {
            incoming,
            responses,
            callbacks,
            state,
        }
    }

    /// Run the splitter.
    ///
    /// Continuously decodes incoming frames and forwards them as responses or callbacks.
    pub async fn run(mut self) {
        while let Some(frame) = self.incoming.decode().await {
            match frame {
                Ok(frame) => {
                    self.handle_frame(frame).await;
                }
                Err(error) => {
                    if self.state.read().is_response_pending() {
                        self.responses
                            .send(Err(error))
                            .await
                            .expect("Response channel should be open. This is a bug.");
                    }
                }
            }
        }
    }

    async fn handle_frame(&self, frame: Frame) {
        let (header, parameters) = frame.into();

        match parameters {
            Parameters::Response(response) => {
                trace!("Forwarding response.");
                self.handle_response(Parameters::Response(response)).await;
            }
            Parameters::Callback(callback) => {
                if header.is_async_callback() {
                    trace!("Forwarding async callback.");
                    self.handle_callback(callback).await;
                } else {
                    trace!("Forwarding non-async callback as response.");
                    self.handle_response(Parameters::Callback(callback)).await;
                }
            }
        }
    }

    async fn handle_response(&self, parameters: Parameters) {
        self.responses
            .send(Ok(parameters))
            .await
            .expect("Response channel should be open. This is a bug.");
    }

    async fn handle_callback(&self, handler: Callback) {
        self.callbacks
            .send(handler)
            .await
            .expect("Callback channel should be open. This is a bug");
    }
}
