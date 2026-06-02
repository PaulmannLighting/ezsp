use std::collections::BTreeMap;
use std::sync::Arc;

use aps::data::Frame;
use log::{debug, error, trace, warn};
use tokio::sync::Mutex;
use tokio::sync::mpsc::{Receiver, Sender};
use zigbee_hw::Event;

use crate::defragmentation::{Defragment, Transaction};
use crate::frame::parameters::networking::handler::Handler as Networking;
use crate::parameters::messaging::handler::{Handler as Messaging, IncomingMessage, MessageSent};
use crate::parameters::security::handler::Handler as Security;
use crate::parameters::trust_center::handler::{Handler as TrustCenter, TrustCenterJoin};
use crate::{Callback, ember};

/// Type alias for a thread-safe list of callback handlers.
///
/// TODO: Refactor this away by e.g. using the actor model.
pub type Events = Arc<Mutex<Vec<Sender<Event>>>>;

/// Handler for processing incoming messages.
#[derive(Debug)]
pub struct MessageHandler {
    handlers: Events,
    transactions: BTreeMap<u8, Transaction>,
}

impl MessageHandler {
    /// Creates a new `MessageHandler` with the given outgoing channel size.
    #[must_use]
    pub const fn new(handlers: Events) -> Self {
        Self {
            handlers,
            transactions: BTreeMap::new(),
        }
    }

    /// Processes incoming messages and sends events to the outgoing channel.
    pub async fn run(mut self, mut callbacks: Receiver<Callback>) {
        while let Some(callback) = callbacks.recv().await {
            self.process_callback(callback).await;
        }

        warn!("Callback channel closed.");
    }

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

    async fn handle_networking_callbacks(&self, networking: Networking) {
        match networking {
            Networking::StackStatus(status) => self.handle_stack_status(status.result()).await,
            Networking::ChildJoin(child_join) => {
                self.forward_to_handlers(child_join.into()).await;
            }
            Networking::NetworkFound(_) => {
                trace!("Network found events are handled by the network scanner.");
            }
            Networking::ScanComplete(_) => {
                trace!("Scan complete events are handled by the network scanner.");
            }
            other => {
                warn!("Received unsupported networking callback: {other:?}");
            }
        }
    }

    async fn handle_messaging_callbacks(&mut self, messaging: Messaging) {
        match messaging {
            Messaging::IncomingMessage(incoming_message) => {
                self.handle_incoming_message(incoming_message).await
            }
            Messaging::MessageSent(message_sent) => {
                Self::handle_message_sent(&message_sent);
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

    async fn handle_incoming_message(&mut self, incoming_message: IncomingMessage) {
        debug!("Incoming message: {incoming_message:?}");

        let defragmented_message = match self.transactions.defragment(incoming_message) {
            Ok(Some(defragmented_message)) => {
                trace!("Defragmented frame: {defragmented_message:?}");
                defragmented_message
            }
            Ok(None) => {
                warn!("Frame defragmentation incomplete.");
                return;
            }
            Err(error) => {
                error!("Defragmentation error: {error}");
                return;
            }
        };

        let src_address = defragmented_message.sender();

        match Frame::try_from(defragmented_message) {
            Ok(aps_frame) => {
                self.forward_to_handlers(Event::MessageReceived {
                    src_address,
                    aps_frame: aps_frame.into(),
                })
                .await
            }
            Err(error) => {
                warn!("Ignoring unknown APS frame type: {error}");
            }
        }
    }

    /// Forward EZSP callbacks to registered handlers.
    async fn forward_to_handlers(&self, event: Event) {
        let mut lock = self.handlers.lock().await;

        trace!("Removing closed EZSP event handlers...");
        lock.retain(|sender| !sender.is_closed());

        for sender in lock.iter() {
            trace!("Forwarding EZSP event to registered handler: {sender:?}");
            if let Err(error) = sender.send(event.clone()).await {
                trace!("Failed to forward EZSP event to registered handler: {error}");
            }
        }
    }

    fn handle_message_sent(message_sent: &MessageSent) {
        match message_sent.ack_received() {
            Ok(true) => debug!("ACK received for sent message: {message_sent}"),
            Ok(false) => warn!("No ACK received for sent message: {message_sent}"),
            Err(error) => error!("{error}: {message_sent}"),
        }
    }

    async fn handle_stack_status(&self, status: Result<ember::Status, u8>) {
        let status = match status {
            Ok(status) => status,
            Err(value) => {
                error!("Received invalid stack status: {value}");
                return;
            }
        };

        match status.try_into() {
            Ok(event) => self.forward_to_handlers(event).await,
            Err(status) => {
                warn!("Received unhandled stack status: {status:?}");
            }
        }
    }

    async fn handle_trust_center_join(&self, trust_center_join: TrustCenterJoin) {
        match trust_center_join.try_into() {
            Ok(event) => self.forward_to_handlers(event).await,
            Err(handler) => {
                warn!("Ignoring trust center join event with invalid status: {handler:?}");
            }
        };
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
