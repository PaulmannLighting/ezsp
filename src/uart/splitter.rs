use std::fmt::Debug;
use std::io;

use log::{trace, warn};
use tokio::sync::mpsc::Sender;

use super::decoder::Decoder;
use crate::error::Error;
use crate::frame::{Callback, Frame, Parameters};

/// Splits incoming EZSP frames into responses and asynchronous callbacks.
///
/// On EZSP-UART, callbacks are asynchronous by default. The response frame
/// control byte identifies whether a callback arrived asynchronously or as part
/// of a normal command/response exchange.
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
    ///
    /// # Errors
    ///
    /// Returns an error if the response or callback destination channel closes.
    pub async fn run(mut self) -> io::Result<()> {
        while let Some(frame) = self.incoming.decode().await {
            match frame {
                Ok(frame) => {
                    trace!("Received frame: {frame:?}");
                    self.handle_frame(frame).await?;
                }
                Err(error) => {
                    warn!("Failed to decode frame: {error}");
                    self.responses
                        .send(Err(error))
                        .await
                        .map_err(|_| io::Error::from(io::ErrorKind::BrokenPipe))?;
                }
            }
        }

        Ok(())
    }

    async fn handle_frame(&self, frame: Frame) -> io::Result<()> {
        let (header, parameters) = frame.into();

        match parameters {
            Parameters::Response(response) => {
                trace!("Forwarding response: {response:?}");
                self.responses
                    .send(Ok(Parameters::Response(response)))
                    .await
                    .map_err(|_| io::Error::from(io::ErrorKind::BrokenPipe))
            }
            Parameters::Callback(callback) => {
                if header.is_async_callback() {
                    trace!("Forwarding async callback: {callback:?}");
                    self.callbacks
                        .send(callback)
                        .await
                        .map_err(|_| io::Error::from(io::ErrorKind::BrokenPipe))
                } else {
                    trace!("Forwarding non-async callback as response: {callback:?}");
                    self.responses
                        .send(Ok(Parameters::Callback(callback)))
                        .await
                        .map_err(|_| io::Error::from(io::ErrorKind::BrokenPipe))
                }
            }
        }
    }
}
