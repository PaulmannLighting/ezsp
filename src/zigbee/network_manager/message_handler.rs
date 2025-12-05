use log::{error, trace, warn};
use tokio::sync::mpsc::Receiver;
use tokio_mpmc::ChannelError;
use zigbee_nwk::Event;

use crate::ember::device::Update;
use crate::frame::parameters::networking::handler::Handler as Networking;
use crate::parameters::messaging::handler::{Handler as Messaging, IncomingMessage, MessageSent};
use crate::parameters::networking::handler::ChildJoin;
use crate::parameters::trust_center::handler::{Handler as TrustCenter, TrustCenterJoin};
use crate::{Callback, ember};

/// Handler for processing incoming messages.
///
/// TODO: Handle and reassemble fragmented frames.
pub struct MessageHandler {
    outgoing: tokio_mpmc::Sender<Event>,
}

impl MessageHandler {
    /// Creates a new `MessageHandler` with the given outgoing channel size.
    pub fn new(size: usize) -> (Self, tokio_mpmc::Receiver<Event>) {
        let (outgoing, rx) = tokio_mpmc::channel(size);
        (Self { outgoing }, rx)
    }

    pub async fn process(self, mut callbacks: Receiver<Callback>) {
        while let Some(callback) = callbacks.recv().await {
            self.handle(callback).await.unwrap_or_else(|error| {
                error!("Failed to send event: {error}");
            });
        }

        warn!("Callback channel closed.");
    }

    async fn handle(&self, callback: Callback) -> Result<(), ChannelError> {
        match callback {
            Callback::Messaging(Messaging::IncomingMessage(incoming_message)) => {
                self.handle_incoming_message(incoming_message).await
            }
            Callback::Messaging(Messaging::MessageSent(message_sent)) => {
                Self::handle_message_sent(&message_sent);
                Ok(())
            }
            Callback::Networking(Networking::StackStatus(status)) => {
                self.handle_stack_status(status.result()).await
            }
            Callback::Networking(Networking::ChildJoin(child_join)) => {
                self.handle_child_join(child_join).await
            }
            Callback::TrustCenter(TrustCenter::TrustCenterJoin(trust_center_join)) => {
                self.handle_trust_center_join(trust_center_join).await
            }
            other => {
                // TODO: Handle other callbacks.
                warn!("Received unsupported callback: {other:?}");
                Ok(())
            }
        }
    }

    async fn handle_incoming_message(
        &self,
        incoming_message: IncomingMessage,
    ) -> Result<(), ChannelError> {
        match incoming_message.try_into() {
            Ok(received_aps_frame) => {
                self.outgoing
                    .send(Event::MessageReceived(received_aps_frame))
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
        trace!("Message sent: {message_sent:?}");
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
                error!("Received invalid trust center join status: {value}");
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
}
