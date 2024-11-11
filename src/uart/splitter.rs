use log::trace;
use tokio::sync::mpsc::Sender;

use crate::error::Error;
use crate::frame::{Callback, Frame, Parameters};
use crate::uart::decoder::Decoder;

/// Split incoming `EZSP` frames into responses and asynchronous callbacks.
#[derive(Debug)]
pub struct Splitter {
    incoming: Decoder,
    responses: Sender<Result<Parameters, Error>>,
    callbacks: Sender<Callback>,
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
    ) -> Self {
        Self {
            incoming,
            responses,
            callbacks,
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
                    if self.incoming.state.requests_pending() {
                        self.responses
                            .send(Err(error))
                            .await
                            .expect("Failed to send response.");
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
            .expect("Failed to send response.");
    }

    async fn handle_callback(&self, handler: Callback) {
        self.callbacks
            .send(handler)
            .await
            .expect("Failed to send callback.");
    }
}
