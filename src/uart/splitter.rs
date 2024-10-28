use log::{error, trace};
use tokio::sync::mpsc::Sender;

use crate::uart::decoder::Decoder;
use crate::{Frame, Handler, Parameters};

#[derive(Debug)]
pub struct Splitter {
    incoming: Decoder,
    responses: Sender<Parameters>,
    callbacks: Sender<Handler>,
}

impl Splitter {
    pub fn new(
        incoming: Decoder,
        responses: Sender<Parameters>,
        callbacks: Sender<Handler>,
    ) -> Self {
        Self {
            incoming,
            responses,
            callbacks,
        }
    }

    pub async fn run(mut self) {
        while let Some(frame) = self.incoming.decode().await {
            match frame {
                Ok(frame) => {
                    self.handle_frame(frame).await;
                }
                Err(error) => {
                    error!("Failed to decode frame: {error}");
                }
            }
        }
    }

    async fn handle_frame(&mut self, frame: Frame) {
        let is_async_callback = frame.header().is_async_callback();

        match frame.parameters() {
            Parameters::Response(response) => {
                trace!("Forwarding response.");
                self.handle_response(Parameters::Response(response)).await;
            }
            Parameters::Handler(handler) => {
                if is_async_callback {
                    trace!("Forwarding async callback.");
                    self.handle_callback(handler).await;
                } else {
                    trace!("Forwarding non-async callback as response.");
                    self.handle_response(Parameters::Handler(handler)).await;
                }
            }
        }
    }

    async fn handle_response(&mut self, parameters: Parameters) {
        if let Err(error) = self.responses.send(parameters).await {
            error!("Failed to send response: {error}");
        }
    }

    async fn handle_callback(&mut self, handler: Handler) {
        if let Err(error) = self.callbacks.send(handler).await {
            error!("Failed to send callback: {error}");
        }
    }
}
