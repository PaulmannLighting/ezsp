use std::sync::Arc;

use log::{debug, error, trace, warn};
use tokio::sync::Mutex;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_mpmc::ChannelError;
use zigbee::Endpoint;
use zigbee_nwk::Event;

use crate::defragmentation::Defragmenter;
use crate::ember::device::Update;
use crate::frame::parameters::networking::handler::Handler as Networking;
use crate::parameters::messaging::handler::{Handler as Messaging, IncomingMessage, MessageSent};
use crate::parameters::networking::handler::ChildJoin;
use crate::parameters::security::handler::Handler as Security;
use crate::parameters::trust_center::handler::{Handler as TrustCenter, TrustCenterJoin};
use crate::{Callback, ember};

/// Type alias for a thread-safe list of callback handlers.
pub type Handlers = Arc<Mutex<Vec<Sender<Callback>>>>;

/// Handler for processing incoming messages.
pub struct MessageHandler {
    handlers: Handlers,
    outgoing: tokio_mpmc::Sender<Event>,
    transactions: Defragmenter<2>,
}

impl MessageHandler {
    /// Creates a new `MessageHandler` with the given outgoing channel size.
    pub fn new(size: usize) -> (Self, Handlers, tokio_mpmc::Receiver<Event>) {
        let handlers = Arc::new(Mutex::new(Vec::new()));
        let (outgoing, rx) = tokio_mpmc::channel(size);
        (
            Self {
                handlers: handlers.clone(),
                outgoing,
                transactions: Defragmenter::default(),
            },
            handlers,
            rx,
        )
    }

    /// Processes incoming messages and sends events to the outgoing channel.
    pub async fn process(mut self, mut callbacks: Receiver<Callback>) {
        while let Some(callback) = callbacks.recv().await {
            self.forward_to_handlers(&callback).await;
            self.forward_to_zigbee(callback)
                .await
                .unwrap_or_else(|error| {
                    error!("Failed to forward Zigbee event: {error}");
                });
        }

        warn!("Callback channel closed.");
    }

    /// Forward EZSP callbacks to registered handlers.
    async fn forward_to_handlers(&self, callback: &Callback) {
        let mut lock = self.handlers.lock().await;

        trace!("Removing closed EZSP event handlers...");
        lock.retain(|sender| !sender.is_closed());

        for sender in lock.iter() {
            trace!("Forwarding EZSP event to registered handler: {sender:?}");
            if let Err(error) = sender.send(callback.clone()).await {
                trace!("Failed to forward EZSP event to registered handler: {error}");
            }
        }
    }

    /// Translates EZSP callbacks into Zigbee events and sends them to the outgoing channel.
    async fn forward_to_zigbee(&mut self, callback: Callback) -> Result<(), ChannelError> {
        match callback {
            Callback::Messaging(messaging) => self.handle_messaging_callbacks(messaging).await,
            Callback::Networking(networking) => self.handle_networking_callbacks(networking).await,
            Callback::TrustCenter(TrustCenter::TrustCenterJoin(trust_center_join)) => {
                self.handle_trust_center_join(trust_center_join).await
            }
            Callback::Security(security) => {
                Self::handle_security_callbacks(security);
                Ok(())
            }
            other => {
                // TODO: Handle other callbacks.
                warn!("Received unsupported callback: {other:?}");
                Ok(())
            }
        }
    }

    async fn handle_networking_callbacks(
        &self,
        networking: Networking,
    ) -> Result<(), ChannelError> {
        match networking {
            Networking::StackStatus(status) => self.handle_stack_status(status.result()).await,
            Networking::ChildJoin(child_join) => self.handle_child_join(child_join).await,
            Networking::NetworkFound(_) => {
                trace!("Network found events are handled by the network scanner.");
                Ok(())
            }
            Networking::ScanComplete(_) => {
                trace!("Scan complete events are handled by the network scanner.");
                Ok(())
            }
            other => {
                warn!("Received unsupported networking callback: {other:?}");
                Ok(())
            }
        }
    }

    async fn handle_messaging_callbacks(
        &mut self,
        messaging: Messaging,
    ) -> Result<(), ChannelError> {
        match messaging {
            Messaging::IncomingMessage(incoming_message) => {
                self.handle_incoming_message(incoming_message).await
            }
            Messaging::MessageSent(message_sent) => {
                Self::handle_message_sent(&message_sent);
                Ok(())
            }
            Messaging::IncomingSenderEui64(incoming_sender_eui64) => {
                trace!(
                    "Incoming sender EUI64 events are handled by the address manager: {incoming_sender_eui64:?}"
                );
                Ok(())
            }
            other => {
                warn!("Received unsupported messaging callback: {other:?}");
                Ok(())
            }
        }
    }

    async fn handle_incoming_message(
        &mut self,
        incoming_message: IncomingMessage,
    ) -> Result<(), ChannelError> {
        debug!("Incoming message: {incoming_message:?}");

        let defragmented_message = match self.transactions.defragment(incoming_message) {
            Ok(Some(defragmented_message)) => {
                trace!("Defragmented frame: {defragmented_message:?}");
                defragmented_message
            }
            Ok(None) => {
                warn!("Frame defragmentation incomplete.");
                return Ok(());
            }
            Err(error) => {
                error!("Defragmentation error: {error}");
                return Ok(());
            }
        };

        let src_address = defragmented_message.sender();
        let src_endpoint: Endpoint = defragmented_message.aps_frame().source_endpoint().into();

        match defragmented_message.try_into() {
            Ok(command) => {
                self.outgoing
                    .send(Event::MessageReceived {
                        src_address,
                        src_endpoint,
                        command: Box::new(command),
                    })
                    .await
            }
            Err(error) => {
                warn!("Ignoring unknown APS frame type: {error}");
                Ok(())
            }
        }
    }

    fn handle_message_sent(message_sent: &MessageSent) {
        // TODO: Maybe handle those?
        debug!("Message sent: {message_sent:?}");
    }

    async fn handle_stack_status(
        &self,
        status: Result<ember::Status, u8>,
    ) -> Result<(), ChannelError> {
        let status = match status {
            Ok(status) => status,
            Err(value) => {
                error!("Received invalid stack status: {value}");
                return Ok(());
            }
        };

        match status {
            ember::Status::NetworkUp => self.outgoing.send(Event::NetworkUp).await,
            ember::Status::NetworkDown => self.outgoing.send(Event::NetworkDown).await,
            ember::Status::NetworkOpened => self.outgoing.send(Event::NetworkOpened).await,
            ember::Status::NetworkClosed => self.outgoing.send(Event::NetworkClosed).await,
            other => {
                warn!("Received unhandled stack status: {other:?}");
                Ok(())
            }
        }
    }

    async fn handle_child_join(&self, child_join: ChildJoin) -> Result<(), ChannelError> {
        self.outgoing
            .send(if child_join.joining() {
                Event::DeviceJoined {
                    ieee_address: child_join.child_eui64(),
                    pan_id: child_join.child_id(),
                }
            } else {
                Event::DeviceLeft {
                    ieee_address: child_join.child_eui64(),
                    pan_id: child_join.child_id(),
                }
            })
            .await
    }

    async fn handle_trust_center_join(
        &self,
        trust_center_join: TrustCenterJoin,
    ) -> Result<(), ChannelError> {
        let status = match trust_center_join.status() {
            Ok(status) => status,
            Err(value) => {
                warn!("Ignoring trust center join event with invalid status: {value}");
                return Ok(());
            }
        };

        self.outgoing
            .send(match status {
                Update::StandardSecurityUnsecuredJoin => Event::DeviceJoined {
                    ieee_address: trust_center_join.new_node_eui64(),
                    pan_id: trust_center_join.new_node_id(),
                },
                Update::StandardSecurityUnsecuredRejoin => Event::DeviceRejoined {
                    ieee_address: trust_center_join.new_node_eui64(),
                    pan_id: trust_center_join.new_node_id(),
                    secured: false,
                },
                Update::StandardSecuritySecuredRejoin => Event::DeviceRejoined {
                    ieee_address: trust_center_join.new_node_eui64(),
                    pan_id: trust_center_join.new_node_id(),
                    secured: true,
                },
                Update::DeviceLeft => Event::DeviceLeft {
                    ieee_address: trust_center_join.new_node_eui64(),
                    pan_id: trust_center_join.new_node_id(),
                },
            })
            .await
    }

    fn handle_security_callbacks(security: Security) {
        match security {
            Security::ZigbeeKeyEstablishment(zigbee_key_establishment) => {
                match zigbee_key_establishment.partner() {
                    Ok(partner) => {
                        debug!("Zigbee key establishment with partner: IEEE Address: {partner}");
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
