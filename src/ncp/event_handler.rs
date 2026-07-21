use std::collections::BTreeMap;

use log::{debug, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;

use crate::ember::Status;
use crate::frame::parameters::networking::handler::Handler as Networking;
use crate::ncp::{Message, Scans};
use crate::parameters::messaging::handler::{Handler as Messaging, IncomingMessage, MessageSent};
use crate::{Callback, Communicate, Defragmenter, TranslatableEvent};

/// Correlates internal callbacks and translates application-facing events.
///
/// The builder runs this handler in a background task. It aggregates scan
/// callbacks, resolves `messageSent` confirmations, reassembles fragmented APS
/// messages, and converts remaining callbacks into the configured output event
/// type.
#[derive(Debug)]
pub struct EventHandler<T, U> {
    defragmenter: Defragmenter<T>,
    output: Sender<U>,
    scans: Scans,
    responses: BTreeMap<u8, oneshot::Sender<Result<Status, u8>>>,
}

impl<T, U> EventHandler<T, U> {
    pub(crate) fn new(transport: T, output: Sender<U>) -> Self {
        Self {
            defragmenter: Defragmenter::new(transport),
            output,
            scans: Scans::default(),
            responses: BTreeMap::new(),
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
                                if let Err(error) = self.output.send(event).await {
                                    trace!(
                                        "Failed to forward EZSP event to registered handler: {error}"
                                    );
                                }
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

    /// Translates EZSP callbacks into Zigbee events and sends them to the outgoing channel.
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
    fn handle_networking_callbacks(&mut self, networking: Networking) -> Option<Networking> {
        match networking {
            Networking::NetworkFound(network_found) => {
                self.scans.add_network(*network_found);
            }
            Networking::EnergyScanResult(energy_scan_result) => {
                self.scans.add_channel(*energy_scan_result);
            }
            Networking::ScanComplete(_) => {
                self.scans.pop();
            }
            other => {
                return Some(other);
            }
        }

        None
    }

    #[must_use]
    async fn handle_messaging_callbacks(&mut self, messaging: Messaging) -> Option<Messaging> {
        match messaging {
            Messaging::IncomingMessage(incoming_message) => {
                self.handle_incoming_message(*incoming_message).await;
            }
            Messaging::MessageSent(message_sent) => {
                self.handle_message_sent(&message_sent);
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

                if let Err(error) = self.output.send(event).await {
                    trace!("Failed to forward EZSP event to registered handler: {error}");
                }
            }
            Err(error) => {
                warn!("{error}");
            }
        }
    }

    fn handle_message_sent(&mut self, message_sent: &MessageSent) {
        if let Some(response) = self.responses.remove(&message_sent.message_tag())
            && let Err(error) = response.send(message_sent.status())
        {
            match error {
                Ok(status) => {
                    warn!("Failed to send message with status: {status}");
                }
                Err(status_code) => {
                    warn!("Failed to send message with status: {status_code:#04x}");
                }
            }
        }
    }
}
