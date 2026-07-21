use std::future::Future;

use log::{error, trace, warn};
use tokio::sync::mpsc;

use crate::frame::Frame;
use crate::parameters::configuration;
use crate::transceiver::Message;
use crate::{Callback, Error, Parameters, Response};

/// Receives decoded frames from a transport-specific inbound stream.
///
/// Implementations also store the negotiated protocol version so transports
/// can switch between legacy and extended EZSP header decoding.
pub trait Receive {
    /// Receives the next decoded frame, or `None` when the input closes.
    fn receive(&mut self) -> impl Future<Output = Option<Frame<Parameters>>> + Send;

    /// Replaces the negotiated EZSP version and returns the previous value.
    fn set_negotiated_version(&mut self, version: u8) -> Option<u8>;
}

/// Routes received EZSP frames to the transmitter actor or callback stream.
#[derive(Debug)]
pub struct Receiver<T> {
    receive: T,
    callbacks: mpsc::Sender<Callback>,
    transmitter: mpsc::Sender<Message>,
}

impl<T> Receiver<T>
where
    T: Receive,
{
    /// Creates a receiver task around a transport-specific inbound half.
    #[must_use]
    pub const fn new(
        receive: T,
        callbacks: mpsc::Sender<Callback>,
        transmitter: mpsc::Sender<Message>,
    ) -> Self {
        Self {
            receive,
            callbacks,
            transmitter,
        }
    }

    async fn handle_frame(&mut self, frame: Frame<Parameters>) -> Result<(), Error> {
        let (header, payload) = frame.into();

        if let Parameters::Response(Response::Configuration(configuration::Response::Version(
            version,
        ))) = &payload
            && let Some(previous_version) = self
                .receive
                .set_negotiated_version(version.protocol_version())
        {
            error!(
                "Replaced previous version {previous_version} with version {}.",
                version.protocol_version()
            );
        }

        match payload {
            Parameters::Response(response) => {
                trace!("Forwarding response: {response:?}");
                self.transmitter
                    .send(Message::Response(Frame::new(
                        header,
                        Parameters::Response(response),
                    )))
                    .await?;
            }
            Parameters::Callback(callback) => {
                if header.is_async_callback() {
                    trace!("Forwarding async callback: {callback:?}");
                    self.callbacks.send(callback).await.unwrap_or_else(|error| {
                        warn!("Callback channel closed: {error}");
                    });
                } else {
                    trace!("Forwarding non-async callback as response: {callback:?}");
                    self.transmitter
                        .send(Message::Response(Frame::new(
                            header,
                            Parameters::Callback(callback),
                        )))
                        .await?;
                }
            }
        }

        Ok(())
    }
}

impl<T> Receiver<T>
where
    T: Receive,
{
    /// Runs until the inbound stream or the transmitter actor channel closes.
    pub async fn run(mut self) {
        while let Some(frame) = self.receive.receive().await {
            if let Err(error) = self.handle_frame(frame).await {
                warn!("{error}");
                return;
            }
        }
    }
}
