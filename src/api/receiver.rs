use std::future::Future;

use log::{error, trace, warn};
use tokio::sync::mpsc;

use crate::api::Message;
use crate::frame::Frame;
use crate::parameters::configuration;
use crate::{Callback, Error, Parameters, Response};

/// Receives decoded frames from a transport-specific inbound stream.
///
/// The transport performs link-layer I/O and EZSP byte decoding before yielding
/// a complete [`Frame<Parameters>`](Frame). The generic receiver actor then
/// routes responses, synchronous callback-shaped responses, and asynchronous
/// callbacks to their respective consumers.
///
/// The receiver actor passes the currently negotiated protocol version to each
/// call so transports can switch between legacy and extended EZSP header
/// decoding. An `ASHv2` implementation decodes one complete `ASHv2` DATA
/// payload into one [`Frame<Parameters>`](Frame), using legacy headers while
/// the supplied version is `None`.
pub trait Receive {
    /// Receives the next decoded frame using `negotiated_version`, or returns
    /// `None` when the input closes.
    ///
    /// The receiver actor passes `None` until it observes the initial `version`
    /// response. Subsequent calls receive `Some(version)` so the transport can
    /// select version-sensitive decoding without storing negotiation state.
    ///
    /// This method has no error result. The transport implementation therefore
    /// owns its malformed-frame policy, for example logging and skipping an
    /// invalid frame or closing the input.
    fn receive(
        &mut self,
        negotiated_version: Option<u8>,
    ) -> impl Future<Output = Option<Frame<Parameters>>> + Send;
}

/// Routes received EZSP frames to the transmitter actor or callback stream.
#[derive(Debug)]
pub struct Receiver<T> {
    receive: T,
    callbacks: mpsc::Sender<Callback>,
    transmitter: mpsc::Sender<Message>,
    negotiated_version: Option<u8>,
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
            negotiated_version: None,
        }
    }

    async fn handle_frame(&mut self, frame: Frame<Parameters>) -> Result<(), Error> {
        let (header, payload) = frame.into();

        if let Parameters::Response(Response::Configuration(configuration::Response::Version(
            version,
        ))) = &payload
            && let Some(previous_version) =
                self.negotiated_version.replace(version.protocol_version())
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
    T: Receive + Send,
{
    /// Runs until the inbound stream or the transmitter actor channel closes.
    pub async fn run(mut self) {
        while let Some(frame) = self.receive.receive(self.negotiated_version).await {
            if let Err(error) = self.handle_frame(frame).await {
                warn!("{error}");
                return;
            }
        }
    }
}
