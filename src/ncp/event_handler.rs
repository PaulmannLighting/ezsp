use std::collections::BTreeMap;

use log::{debug, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;

use crate::ember::Status;
use crate::frame::parameters::networking::handler::Handler as Networking;
use crate::ncp::scans::PendingScan;
use crate::ncp::{Message, Scans};
use crate::parameters::messaging::handler::{Handler as Messaging, IncomingMessage, MessageSent};
use crate::{
    Callback, Communicate, Defragmenter, Networking as NetworkCommands, TranslatableEvent,
};

/// Correlates internal callbacks and translates application-facing events.
///
/// The builder returns this handler's future for the caller to run. The handler
/// aggregates scan callbacks, resolves `messageSent` confirmations, reassembles fragmented APS
/// messages, and converts remaining callbacks into the configured output event
/// type.
#[derive(Debug)]
pub struct EventHandler<T, U> {
    defragmenter: Defragmenter<T>,
    scan_connection: T,
    output: Sender<U>,
    scans: Scans,
    responses: BTreeMap<u8, oneshot::Sender<Result<Status, u8>>>,
}

impl<T, U> EventHandler<T, U> {
    pub(crate) fn new(transport: T, output: Sender<U>) -> Self
    where
        T: Clone,
    {
        Self {
            defragmenter: Defragmenter::new(transport.clone()),
            scan_connection: transport,
            output,
            scans: Scans::default(),
            responses: BTreeMap::new(),
        }
    }

    #[must_use]
    fn handle_networking_callbacks(&mut self, networking: Networking) -> Option<Networking> {
        match networking {
            Networking::NetworkFound(network_found) => {
                self.scans.add_network(*network_found);
            }
            Networking::EnergyScanResult(energy_scan_result) => {
                self.scans.add_channel(*energy_scan_result);
            }
            Networking::ScanComplete(completed) => {
                self.scans.complete(completed.status());
            }
            other => {
                return Some(other);
            }
        }

        None
    }

    fn handle_message_sent(&mut self, message_sent: &MessageSent) -> bool {
        let Some(response) = self.responses.remove(&message_sent.message_tag()) else {
            return false;
        };

        if let Err(error) = response.send(message_sent.status()) {
            match error {
                Ok(status) => {
                    warn!("Failed to send message with status: {status}");
                }
                Err(status_code) => {
                    warn!("Failed to send message with status: {status_code:#04x}");
                }
            }
        }

        true
    }
}

impl<T, U> EventHandler<T, U>
where
    T: Communicate,
{
    /// Registers only accepted commands before processing their queued callbacks.
    async fn start_scan(&mut self, scan: PendingScan, channel_mask: u32, duration: u8) {
        if scan.is_closed() {
            return;
        }
        match self
            .scan_connection
            .start_scan(scan.kind(), channel_mask, duration)
            .await
        {
            Ok(()) => self.scans.register(scan),
            Err(error) => scan.complete(Err(error), Vec::new(), Vec::new()),
        }
    }
}

impl<T, U> EventHandler<T, U>
where
    T: Communicate,
    U: TranslatableEvent,
{
    pub(crate) async fn run(mut self, mut inbox: Receiver<Message>) {
        while let Some(message) = inbox.recv().await {
            match message {
                Message::Callback(callback) => {
                    if let Some(response) = self.process_callback(*callback).await {
                        match response {
                            Ok(event) => {
                                Self::emit_event(&self.output, event).await;
                            }
                            Err(error) => {
                                debug!("Failed to translate event: {error}");
                            }
                        }
                    }
                }
                Message::NetworkScan(sender) => {
                    self.scans.push(sender.into());
                }
                Message::ChannelScan(sender) => {
                    self.scans.push(sender.into());
                }
                Message::StartNetworkScan {
                    channel_mask,
                    duration,
                    response,
                } => {
                    self.start_scan(PendingScan::Network(response), channel_mask, duration)
                        .await;
                }
                Message::StartChannelScan {
                    channel_mask,
                    duration,
                    response,
                } => {
                    self.start_scan(PendingScan::Channel(response), channel_mask, duration)
                        .await;
                }
                Message::Sent { tag, sender } => {
                    if self.responses.insert(tag, sender).is_some() {
                        warn!("Overwrote response channel for message tag: {tag}");
                    }
                }
                Message::Terminate => {
                    trace!("Received termination message.");
                    return;
                }
            }
        }

        warn!("Callback channel closed. Message handler terminating.");
    }

    /// Delivers an application event, logging when its receiver has closed.
    async fn emit_event(output: &Sender<U>, event: U) {
        if let Err(error) = output.send(event).await {
            trace!("Failed to forward EZSP event to registered handler: {error}");
        }
    }

    /// Processes internal callbacks and returns unconsumed callbacks as application events.
    #[must_use]
    async fn process_callback(
        &mut self,
        callback: Callback,
    ) -> Option<Result<U, <U as TryFrom<Callback>>::Error>> {
        match callback {
            Callback::Messaging(messaging) => self
                .handle_messaging_callbacks(messaging)
                .await
                .map(|messaging| U::try_from(Callback::Messaging(messaging))),
            Callback::Networking(networking) => self
                .handle_networking_callbacks(networking)
                .map(|networking| U::try_from(Callback::Networking(networking))),
            other => Some(U::try_from(other)),
        }
    }

    #[must_use]
    async fn handle_messaging_callbacks(&mut self, messaging: Messaging) -> Option<Messaging> {
        match messaging {
            Messaging::IncomingMessage(incoming_message) => {
                self.handle_incoming_message(*incoming_message).await;
            }
            Messaging::MessageSent(message_sent) => {
                if !self.handle_message_sent(&message_sent) {
                    return Some(Messaging::MessageSent(message_sent));
                }
            }
            other => {
                return Some(other);
            }
        }

        None
    }

    async fn handle_incoming_message(&mut self, incoming_message: IncomingMessage) {
        trace!("Incoming message: {incoming_message:?}");
        self.defragmenter.tick();

        let Some(defragmented_message) = self.defragmenter.handle(incoming_message).await else {
            trace!("Message is fragmented. Waiting for more data.");
            return;
        };

        trace!("Message defragmented: {defragmented_message:?}");

        match defragmented_message.try_into() {
            Ok(event) => {
                trace!("Successfully converted defragmented message into an event: {event:?}");

                Self::emit_event(&self.output, event).await;
            }
            Err(error) => {
                warn!("{error}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    use le_stream::{FromLeStream, ToLeStream};
    use tokio::sync::{mpsc, oneshot};

    use super::EventHandler;
    use crate::ember::Status;
    use crate::frame::{Commands, Parameter, RespondsWith};
    use crate::ncp::Message;
    use crate::ncp::scans::PendingScan;
    use crate::parameters::messaging::handler::MessageSent;
    use crate::parameters::networking::handler::{
        EnergyScanResult, Handler as Networking, ScanComplete,
    };
    use crate::parameters::networking::{self, start_scan};
    use crate::{Callback, Communicate, DefragmentedMessage, Error, Parameters, Response};

    const MESSAGE_TAG: u8 = 0x34;
    const APS_SEQUENCE: u8 = 0x56;
    const STATUS_SUCCESS: u8 = 0x00;
    const MESSAGE_SENT_BYTES: [u8; 17] = [
        0x00,
        0x78,
        0x56,
        0x04,
        0x01,
        0x06,
        0x03,
        0x01,
        0x02,
        0x00,
        0x00,
        0x00,
        0x00,
        APS_SEQUENCE,
        MESSAGE_TAG,
        STATUS_SUCCESS,
        0x00,
    ];

    const CHANNEL_MASK: u32 = 1 << 11;
    const DURATION: u8 = 3;
    const CHANNEL: u8 = 11;
    const RSSI: u8 = 0xD8;
    const QUEUE_CAPACITY: usize = 8;
    const COMMAND_SUCCESS: u32 = 0;
    const COMMAND_REJECTED: u32 = u32::MAX;
    const UNKNOWN_COMPLETION: u8 = u8::MAX;

    #[derive(Clone, Debug)]
    struct ScanTransport {
        statuses: VecDeque<u32>,
    }

    impl Communicate for ScanTransport {
        #[expect(
            clippy::unused_async_trait_impl,
            reason = "mock responses are immediately ready"
        )]
        async fn communicate<T>(&mut self, _command: T) -> Result<T::Response, Error>
        where
            T: Parameter + RespondsWith + ToLeStream + Into<Commands>,
        {
            assert_eq!(T::ID, start_scan::Command::ID);
            let status = self.statuses.pop_front().expect("unexpected scan command");
            let response =
                start_scan::Response::from_le_stream(status.to_le_bytes().into_iter()).unwrap();
            let parameters = Parameters::Response(Response::Networking(
                networking::Response::StartScan(Box::new(response)),
            ));
            T::Response::try_from(parameters)
                .map_err(|error| Error::UnexpectedResponse(Box::new(error.into())))
        }
    }

    #[derive(Debug)]
    struct TestEvent;

    impl From<Callback> for TestEvent {
        fn from(_: Callback) -> Self {
            Self
        }
    }

    impl From<DefragmentedMessage> for TestEvent {
        fn from(_: DefragmentedMessage) -> Self {
            Self
        }
    }

    fn ready<F>(future: F) -> F::Output
    where
        F: Future,
    {
        let mut future = pin!(future);
        let mut context = Context::from_waker(Waker::noop());
        match future.as_mut().poll(&mut context) {
            Poll::Ready(result) => result,
            Poll::Pending => panic!("test future unexpectedly blocked"),
        }
    }

    fn energy_result() -> EnergyScanResult {
        EnergyScanResult::from_le_stream([CHANNEL, RSSI].into_iter()).unwrap()
    }

    fn completion(status: u8) -> Networking {
        Networking::ScanComplete(Box::new(
            ScanComplete::from_le_stream([CHANNEL, status].into_iter()).unwrap(),
        ))
    }

    fn message_sent() -> MessageSent {
        MessageSent::from_le_stream(MESSAGE_SENT_BYTES.into_iter())
            .expect("messageSent test callback is complete")
    }

    #[test]
    fn routes_registered_message_sent_to_back_channel() {
        let (output, _events) = mpsc::channel(1);
        let mut handler = EventHandler::<(), ()>::new((), output);
        let (response, result) = oneshot::channel();
        handler.responses.insert(MESSAGE_TAG, response);

        assert!(handler.handle_message_sent(&message_sent()));
        assert_eq!(
            result
                .blocking_recv()
                .expect("response sender is available"),
            Ok(Status::Success)
        );
        assert!(handler.responses.is_empty());
    }

    #[test]
    fn leaves_unregistered_message_sent_for_event_translation() {
        let (output, _events) = mpsc::channel(1);
        let mut handler = EventHandler::<(), ()>::new((), output);

        assert!(!handler.handle_message_sent(&message_sent()));
    }
    #[test]
    fn rejected_scan_does_not_consume_the_next_scans_completion() {
        let (output, _events) = mpsc::channel(QUEUE_CAPACITY);
        let handler = EventHandler::<_, TestEvent>::new(
            ScanTransport {
                statuses: [COMMAND_REJECTED, COMMAND_SUCCESS].into(),
            },
            output,
        );
        let (inbox, messages) = mpsc::channel(QUEUE_CAPACITY);
        let (rejected, rejected_result) = oneshot::channel();
        let (accepted, accepted_result) = oneshot::channel();
        inbox
            .try_send(Message::StartNetworkScan {
                channel_mask: CHANNEL_MASK,
                duration: DURATION,
                response: rejected,
            })
            .unwrap();
        inbox
            .try_send(Message::StartChannelScan {
                channel_mask: CHANNEL_MASK,
                duration: DURATION,
                response: accepted,
            })
            .unwrap();
        inbox
            .try_send(
                Callback::Networking(Networking::EnergyScanResult(Box::new(energy_result())))
                    .into(),
            )
            .unwrap();
        inbox
            .try_send(Callback::Networking(completion(STATUS_SUCCESS)).into())
            .unwrap();
        inbox.try_send(Message::Terminate).unwrap();
        ready(handler.run(messages));

        assert!(matches!(
            rejected_result.blocking_recv().unwrap(),
            Err(Error::Status(crate::Status::Sl(Err(COMMAND_REJECTED))))
        ));
        assert_eq!(
            accepted_result.blocking_recv().unwrap().unwrap(),
            vec![energy_result()]
        );
    }

    #[test]
    fn completion_errors_preserve_status_and_clear_results() {
        for status in [u8::from(Status::InvalidCall), UNKNOWN_COMPLETION] {
            let (output, _events) = mpsc::channel(QUEUE_CAPACITY);
            let mut handler = EventHandler::<_, TestEvent>::new(
                ScanTransport {
                    statuses: [COMMAND_SUCCESS, COMMAND_SUCCESS].into(),
                },
                output,
            );
            let (response, result) = oneshot::channel();
            ready(handler.start_scan(PendingScan::Channel(response), CHANNEL_MASK, DURATION));
            assert!(
                handler
                    .handle_networking_callbacks(Networking::EnergyScanResult(Box::new(
                        energy_result()
                    )))
                    .is_none()
            );
            assert!(
                handler
                    .handle_networking_callbacks(completion(status))
                    .is_none()
            );
            let error = result.blocking_recv().unwrap().unwrap_err();
            assert_eq!(
                error.to_string(),
                Status::check(status).unwrap_err().to_string()
            );

            let (response, result) = oneshot::channel();
            ready(handler.start_scan(PendingScan::Channel(response), CHANNEL_MASK, DURATION));
            assert!(
                handler
                    .handle_networking_callbacks(completion(STATUS_SUCCESS))
                    .is_none()
            );
            assert_eq!(result.blocking_recv().unwrap().unwrap(), []);
        }
    }

    #[test]
    fn canceled_accepted_scan_keeps_its_own_completion() {
        let (output, _events) = mpsc::channel(QUEUE_CAPACITY);
        let mut handler = EventHandler::<_, TestEvent>::new(
            ScanTransport {
                statuses: [COMMAND_SUCCESS, COMMAND_SUCCESS].into(),
            },
            output,
        );
        let (response, result) = oneshot::channel();
        ready(handler.start_scan(PendingScan::Channel(response), CHANNEL_MASK, DURATION));
        drop(result);
        let (response, mut result) = oneshot::channel();
        ready(handler.start_scan(PendingScan::Channel(response), CHANNEL_MASK, DURATION));
        assert!(
            handler
                .handle_networking_callbacks(Networking::EnergyScanResult(
                    Box::new(energy_result())
                ))
                .is_none()
        );
        assert!(
            handler
                .handle_networking_callbacks(completion(STATUS_SUCCESS))
                .is_none()
        );
        assert!(matches!(
            result.try_recv(),
            Err(oneshot::error::TryRecvError::Empty)
        ));
        assert!(
            handler
                .handle_networking_callbacks(completion(STATUS_SUCCESS))
                .is_none()
        );
        assert_eq!(result.blocking_recv().unwrap().unwrap(), []);
    }

    #[test]
    fn skips_requests_canceled_before_the_command_is_issued() {
        let (output, _events) = mpsc::channel(QUEUE_CAPACITY);
        let mut handler = EventHandler::<_, TestEvent>::new(
            ScanTransport {
                statuses: VecDeque::new(),
            },
            output,
        );
        let (response, result) = oneshot::channel();
        drop(result);
        ready(handler.start_scan(PendingScan::Network(response), CHANNEL_MASK, DURATION));
    }
}
