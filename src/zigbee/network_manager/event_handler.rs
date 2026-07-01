use std::collections::BTreeMap;

use aps::data::Frame;
use log::{debug, error, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;
use zigbee_hw::{Event, EventTranslator};

pub use self::message::Message;
use self::scans::Scans;
use crate::Callback;
use crate::ember::Status;
use crate::frame::parameters::networking::handler::Handler as Networking;
use crate::parameters::messaging::handler::{Handler as Messaging, IncomingMessage, MessageSent};
use crate::parameters::security::handler::Handler as Security;
use crate::parameters::trust_center::handler::{Handler as TrustCenter, TrustCenterJoin};

mod message;
mod scans;

/// Actor for processing incoming events.
///
/// This actor actually receives messages of type [`Message`] which can be wrapped raw EZSP events
/// ("Callbacks"), subscription requests or requests to start a channel or network scan.
/// Also, termination signals may be received.
#[derive(Debug)]
pub struct EventHandler {
    output: Sender<Event>,
    scans: Scans,
    responses: BTreeMap<u8, oneshot::Sender<Result<Status, u8>>>,
}

impl EventHandler {
    /// Translates EZSP callbacks into Zigbee events and sends them to the outgoing channel.
    async fn process_callback(&mut self, callback: Callback) {
        match callback {
            Callback::Messaging(messaging) => self.handle_messaging_callbacks(messaging).await,
            Callback::Networking(networking) => self.handle_networking_callbacks(networking).await,
            Callback::TrustCenter(TrustCenter::TrustCenterJoin(trust_center_join)) => {
                self.handle_trust_center_join(trust_center_join).await;
            }
            Callback::Security(security) => {
                Self::handle_security_callbacks(security);
            }
            other => {
                // TODO: Handle other callbacks.
                warn!("Received unsupported callback: {other:?}");
            }
        }
    }

    async fn handle_networking_callbacks(&mut self, networking: Networking) {
        match networking {
            Networking::StackStatus(status) => self.handle_stack_status(status.result()).await,
            Networking::ChildJoin(child_join) => {
                self.forward_event(child_join.into()).await;
            }
            Networking::NetworkFound(network_found) => {
                self.scans.add_network(network_found.into());
            }
            Networking::EnergyScanResult(energy_scan_result) => {
                self.scans.add_channel(energy_scan_result.into());
            }
            Networking::ScanComplete(_) => {
                self.scans.pop();
            }
            other => {
                warn!("Received unsupported networking callback: {other:?}");
            }
        }
    }

    async fn handle_messaging_callbacks(&mut self, messaging: Messaging) {
        match messaging {
            Messaging::IncomingMessage(incoming_message) => {
                self.handle_incoming_message(incoming_message).await;
            }
            Messaging::MessageSent(message_sent) => {
                self.handle_message_sent(&message_sent);
            }
            Messaging::IncomingSenderEui64(incoming_sender_eui64) => {
                trace!(
                    "Incoming sender EUI64 events are handled by the address manager: {incoming_sender_eui64:?}"
                );
            }
            other => {
                warn!("Received unsupported messaging callback: {other:?}");
            }
        }
    }

    async fn handle_incoming_message(&self, incoming_message: IncomingMessage) {
        debug!("Incoming message: {incoming_message:?}");

        let src_address = incoming_message.sender();

        match Frame::try_from(incoming_message) {
            Ok(aps_frame) => {
                self.forward_event(Event::MessageReceived {
                    src_address,
                    aps_frame,
                })
                .await;
            }
            Err(error) => {
                warn!("Ignoring malformed APS frame: {error}");
            }
        }
    }

    /// Forward EZSP callbacks to registered handlers.
    async fn forward_event(&self, event: Event) {
        if let Err(error) = self.output.send(event).await {
            trace!("Failed to forward EZSP event to registered handler: {error}");
        }
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

    async fn handle_stack_status(&self, status: Result<Status, u8>) {
        let status = match status {
            Ok(status) => status,
            Err(value) => {
                error!("Received invalid stack status: {value}");
                return;
            }
        };

        match status.try_into() {
            Ok(event) => self.forward_event(event).await,
            Err(status) => {
                warn!("Received unhandled stack status: {status:?}");
            }
        }
    }

    async fn handle_trust_center_join(&self, trust_center_join: TrustCenterJoin) {
        match trust_center_join.try_into() {
            Ok(event) => self.forward_event(event).await,
            Err(handler) => {
                warn!("Ignoring trust center join event with invalid status: {handler:?}");
            }
        }
    }

    fn handle_security_callbacks(security: Security) {
        match security {
            Security::ZigbeeKeyEstablishment(zigbee_key_establishment) => {
                match zigbee_key_establishment.partner() {
                    Ok(partner) => {
                        debug!("Zigbee key establishment with partner: IEEE address: {partner}");
                    }
                    Err(value) => {
                        warn!(
                            "Zigbee key establishment with invalid partner IEEE address: {value:?}"
                        );
                    }
                }
            }
            Security::SwitchNetworkKey(switch_network_key) => {
                debug!(
                    "Switch network key. New key sequence: {}",
                    switch_network_key.sequence_number()
                );
            }
        }
    }
}
impl EventTranslator for EventHandler {
    type Input = Message;

    fn new(output: Sender<Event>) -> Self {
        Self {
            output,
            scans: Scans::default(),
            responses: BTreeMap::new(),
        }
    }

    async fn run(mut self, mut callbacks: Receiver<Self::Input>) {
        while let Some(message) = callbacks.recv().await {
            match message {
                Message::Callback(callback) => {
                    self.process_callback(*callback).await;
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
}
