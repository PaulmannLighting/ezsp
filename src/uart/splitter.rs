use log::{error, trace};
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
    pub async fn run(mut self) {
        while let Some(frame) = self.incoming.decode().await {
            match frame {
                Ok(frame) => {
                    self.handle_frame(frame).await;
                }
                Err(error) => {
                    error!("Failed to decode frame: {error}");

                    if self.incoming.state.requests_pending() {
                        if let Err(error) = self.responses.send(Err(error)).await {
                            error!("Failed to send error: {error}");
                        }
                    }
                }
            }
        }
    }

    async fn handle_frame(&self, frame: Frame) {
        let is_async_callback = frame.header().is_async_callback();

        match frame.parameters() {
            Parameters::Response(response) => {
                trace!("Forwarding response.");
                self.handle_response(Parameters::Response(response)).await;
            }
            Parameters::Callback(callback) => {
                if is_async_callback {
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
        if let Err(error) = self.responses.send(Ok(parameters)).await {
            error!("Failed to send response: {error}");
        }
    }

    async fn handle_callback(&self, handler: Callback) {
        if let Err(error) = self.callbacks.send(handler).await {
            error!("Failed to send callback: {error}");
        }
    }
}
