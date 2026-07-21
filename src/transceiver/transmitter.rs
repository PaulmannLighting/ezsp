use std::collections::BTreeMap;
use std::num::{NonZero, TryFromIntError};

use log::{debug, error, info, trace, warn};
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot::Sender;

use crate::frame::{Commands, Parameter};
use crate::parameters::configuration;
use crate::parameters::configuration::version;
use crate::parameters::configuration::version::Command as VersionCommand;
use crate::transceiver::Message;
use crate::{
    Command, Error, Extended, Frame, Header, Legacy, MIN_NON_LEGACY_VERSION, Parameters, Response,
    ValueError,
};

type PendingNegotiation = (NonZero<u8>, Sender<Result<(), Error>>);

/// Sends fully framed commands through a transport-specific outbound sink.
pub trait Transmit {
    /// Transmits one complete EZSP command frame.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] when the frame cannot be encoded or sent.
    fn transmit(
        &mut self,
        frame: Frame<Commands>,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

/// Actor that serializes commands and correlates sequence-numbered responses.
///
/// The actor owns the transport-specific transmitter, assigns EZSP sequence
/// numbers, tracks pending one-shot responses, and performs the initial version
/// negotiation.
pub struct Transmitter<T> {
    transmit: T,
    inbox: Receiver<Message>,
    negotiated_version: Option<u8>,
    pending_responses: BTreeMap<u8, Sender<Result<Parameters, Error>>>,
    version_negotiation: Option<PendingNegotiation>,
    sequence: u8,
}

impl<T> Transmitter<T> {
    /// Creates a transmitter actor reading from `inbox`.
    #[must_use]
    pub const fn new(transmit: T, inbox: Receiver<Message>) -> Self {
        Self {
            transmit,
            inbox,
            negotiated_version: None,
            pending_responses: BTreeMap::new(),
            version_negotiation: None,
            sequence: 0,
        }
    }

    fn header(&self, id: u16) -> Result<Header, TryFromIntError> {
        let header = if self
            .negotiated_version
            .is_some_and(|version| version >= MIN_NON_LEGACY_VERSION)
        {
            Header::Extended(Extended::new(self.sequence, Command::default().into(), id))
        } else {
            Header::Legacy(Legacy::new(
                self.sequence,
                Command::default().into(),
                id.try_into()?,
            ))
        };

        Ok(header)
    }

    fn handle_response(&mut self, frame: Frame<Parameters>) {
        let (header, payload) = frame.into();

        if let Parameters::Response(Response::Configuration(configuration::Response::Version(
            version,
        ))) = payload
        {
            self.handle_negotiated_version(*version);
            return;
        }

        let Some(response_channel) = self.pending_responses.remove(&header.sequence()) else {
            warn!(
                "Received response for unknown sequence: {}",
                header.sequence()
            );
            return;
        };

        response_channel
            .send(Ok(payload))
            .unwrap_or_else(|parameters| {
                debug!("Response channel closed for request #{}", header.sequence());
                trace!("Response was: {parameters:?}");
            });
    }

    fn handle_negotiated_version(&mut self, negotiated: version::Response) {
        trace!("Received negotiated version response: {negotiated:?}");

        let Some((desired_version, response)) = self.version_negotiation.take() else {
            error!("Received negotiated version without a desired version.");
            return;
        };

        if desired_version.get() != negotiated.protocol_version() {
            response
                .send(Err(Error::ProtocolVersionMismatch {
                    desired: desired_version.get(),
                    negotiated,
                }))
                .unwrap();
            return;
        }

        if let Some(previous_version) = self
            .negotiated_version
            .replace(negotiated.protocol_version())
        {
            info!(
                "Changed negotiated version from {previous_version} to {}.",
                negotiated.protocol_version()
            );
        }

        response.send(Ok(())).unwrap_or_else(drop);
    }
}

impl<T> Transmitter<T>
where
    T: Transmit,
{
    /// Runs the actor until every sender for its inbox has been dropped.
    pub async fn run(mut self) {
        while let Some(message) = self.inbox.recv().await {
            match message {
                Message::Connect {
                    desired_version,
                    response,
                } => self.connect(desired_version, response).await,
                Message::Command { command, response } => {
                    self.handle_command(command, response).await;
                }
                Message::Response(frame) => {
                    self.handle_response(frame);
                }
            }
        }
    }

    async fn connect(&mut self, desired_version: NonZero<u8>, response: Sender<Result<(), Error>>) {
        trace!("Establishing connection with desired version: {desired_version:?}");

        let header = self
            .header(VersionCommand::ID)
            .expect("Version command ID fits into a u16.");
        let command = VersionCommand::new(desired_version.get());

        if let Err(error) = self
            .transmit
            .transmit(Frame::new(header, command.into()))
            .await
        {
            response.send(Err(error)).unwrap_or_else(drop);
            return;
        }

        self.version_negotiation
            .replace((desired_version, response));
        self.sequence = self.sequence.wrapping_add(1);
    }

    async fn handle_command(
        &mut self,
        command: Commands,
        response: Sender<Result<Parameters, Error>>,
    ) {
        let header = match self.header(command.id()) {
            Ok(header) => header,
            Err(error) => {
                response
                    .send(Err(ValueError::InvalidFrameId(error).into()))
                    .unwrap_or_else(drop);
                return;
            }
        };

        if self.pending_responses.contains_key(&header.sequence()) {
            response
                .send(Err(Error::TransactionQueueFull))
                .unwrap_or_else(drop);
            return;
        }

        if let Err(error) = self.transmit.transmit(Frame::new(header, command)).await {
            response.send(Err(error)).unwrap_or_else(drop);
            return;
        }

        self.pending_responses.insert(header.sequence(), response);
        self.sequence = self.sequence.wrapping_add(1);
    }
}
