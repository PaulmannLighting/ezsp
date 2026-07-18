use std::collections::BTreeMap;

use log::{debug, error, trace, warn};
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use crate::Callback;
use crate::ember::Status;
use crate::frame::parameters::networking::handler::Handler as Networking;
pub use crate::ncp::Scans;
use crate::ncp::messages::{ToReceiver, ToTransmitter};
use crate::parameters::messaging::handler::{Handler as Messaging, IncomingMessage, MessageSent};

pub struct Receiver {
    transmitter: Sender<ToTransmitter>,
    output: Sender<Callback>,
    scans: Scans,
    responses: BTreeMap<u8, oneshot::Sender<Result<Status, u8>>>,
}

impl Receiver {
    pub const fn new(transmitter: Sender<ToTransmitter>, output: Sender<Callback>) -> Self {
        Self {
            transmitter,
            output,
            scans: Scans::new(),
            responses: BTreeMap::new(),
        }
    }

    pub async fn run(mut self, mut inbox: tokio::sync::mpsc::Receiver<ToReceiver>) {
        while let Some(message) = inbox.recv().await {
            match message {
                ToReceiver::Callback(callback) => {
                    self.process_callback(*callback).await;
                }
                ToReceiver::NetworkScan(sender) => {
                    self.scans.push(sender.into());
                }
                ToReceiver::ChannelScan(sender) => {
                    self.scans.push(sender.into());
                }
                ToReceiver::Sent { tag, sender } => {
                    if self.responses.insert(tag, sender).is_some() {
                        warn!("Overwrote response channel for message tag: {tag}");
                    }
                }
                ToReceiver::Terminate => {
                    trace!("Received termination message.");
                    return;
                }
            }
        }

        warn!("Callback channel closed. Message handler terminating.");
    }

    async fn process_callback(&mut self, callback: Callback) {
        match callback {
            Callback::Messaging(messaging) => self.handle_messaging_callbacks(messaging).await,
            Callback::Networking(networking) => self.handle_networking_callbacks(networking).await,
            other => {
                self.forward_callback(other).await;
            }
        }
    }

    async fn handle_networking_callbacks(&mut self, networking: Networking) {
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
                self.forward_callback(Callback::Networking(other)).await;
            }
        }
    }

    async fn handle_messaging_callbacks(&mut self, messaging: Messaging) {
        match messaging {
            Messaging::IncomingMessage(incoming_message) => {
                self.handle_incoming_message(*incoming_message).await;
            }
            Messaging::MessageSent(message_sent) => {
                self.handle_message_sent(&message_sent);
            }
            other => {
                self.forward_callback(Callback::Messaging(other)).await;
            }
        }
    }

    async fn handle_incoming_message(&self, incoming_message: IncomingMessage) {
        debug!("Incoming message: {incoming_message:?}");

        if incoming_message.aps_frame().fragmentation().is_some() {
            self.transmitter
                .send(ToTransmitter::SendReply {
                    node_id: incoming_message.sender(),
                    aps_frame: incoming_message.aps_frame().clone(),
                    payload: Box::new(incoming_message.message().iter().copied().collect()),
                })
                .await
                .unwrap_or_else(|error| {
                    error!("Failed to send reply via transmitter handle: {error}");
                });
        }

        self.forward_callback(Callback::Messaging(Messaging::IncomingMessage(
            incoming_message.into(),
        )))
        .await;
    }

    fn handle_message_sent(&mut self, message_sent: &MessageSent) {
        if let Some(response) = self.responses.remove(&message_sent.message_tag())
            && let Err(error) = response.send(message_sent.status())
        {
            warn!("Failed to send message sent status: {error:?}");
        }

        match message_sent.ack_received() {
            Ok(true) => trace!("ACK received for sent message: {message_sent}"),
            Ok(false) => warn!("No ACK received for sent message: {message_sent}"),
            Err(error) => error!("{error}: {message_sent}"),
        }
    }

    async fn forward_callback(&self, callback: Callback) {
        self.output.send(callback).await.unwrap_or_else(|error| {
            error!("Failed to send callback: {error}");
        });
    }
}
