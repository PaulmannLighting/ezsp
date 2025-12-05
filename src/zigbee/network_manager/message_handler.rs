mod fragments;

use std::collections::BTreeMap;

use aps::Control;
use le_stream::FromLeStream;
use log::{debug, error, info, warn};
use tokio::sync::mpsc::Receiver;
use zigbee_nwk::{Event, ReceivedApsFrame};

use crate::ember::device::Update;
use crate::ember::message::Incoming;
use crate::frame::parameters::networking::handler::Handler as Networking;
use crate::parameters::messaging::handler::{Handler as Messaging, IncomingMessage};
use crate::parameters::trust_center::handler::{Handler as TrustCenter, TrustCenterJoin};
use crate::zigbee::network_manager::message_handler::fragments::Fragments;
use crate::{Callback, ember};

/// Handler for processing incoming messages.
pub struct MessageHandler {
    outgoing: tokio_mpmc::Sender<Event>,
    // TODO: Handle and reassemble fragmented frames.
    _fragments: BTreeMap<u8, Fragments>,
}

impl MessageHandler {
    /// Creates a new `MessageHandler` with the given outgoing channel size.
    pub fn new(size: usize) -> (Self, tokio_mpmc::Receiver<Event>) {
        let (outgoing, rx) = tokio_mpmc::channel(size);

        (
            Self {
                outgoing,
                _fragments: BTreeMap::default(),
            },
            rx,
        )
    }

    pub async fn process(self, mut callbacks: Receiver<Callback>) {
        while let Some(callback) = callbacks.recv().await {
            self.handle(callback).await;
        }

        warn!("Callback channel closed.");
    }

    async fn handle(&self, callback: Callback) {
        match callback {
            Callback::Messaging(Messaging::IncomingMessage(incoming_message)) => {
                self.handle_incoming_message(incoming_message);
            }

            Callback::Networking(Networking::StackStatus(status)) => {
                self.handle_stack_status(status.result()).await;
            }
            Callback::TrustCenter(TrustCenter::TrustCenterJoin(trust_center_join)) => {
                self.handle_trust_center_join(trust_center_join).await;
            }
            other => {
                // TODO: Handle other callbacks.
                warn!("Received unsupported callback: {other:?}");
            }
        }
    }

    fn handle_incoming_message(&self, incoming_message: IncomingMessage) {
        let typ = match incoming_message.typ() {
            Ok(typ) => typ,
            Err(value) => {
                error!("Received incoming message with invalid type: {value}");
                return;
            }
        };

        match typ {
            Incoming::Broadcast | Incoming::Unicast | Incoming::UnicastReply => {
                info!("Handling incoming message: {incoming_message:?}");
            }
            Incoming::BroadcastLoopback | Incoming::MulticastLoopback => {
                warn!("Ignoring loopback message: {incoming_message:?}");
                return;
            }
            Incoming::Multicast => {
                info!("Handling multicast message: {incoming_message:?}");
            }
            Incoming::ManyToOneRouteRequest => {
                warn!("Ignoring many-to-one route request message: {incoming_message:?}");
                return;
            }
        }

        let payload = incoming_message.into_message();

        if let Some(first_byte) = payload.first() {
            let control = Control::from_bits_retain(*first_byte);
            debug!("Control fields:");
            for (name, control) in control.iter_names() {
                debug!(" - {name}: {control:#04X}");
            }
        }

        match ReceivedApsFrame::from_le_stream_exact(payload.into_iter()) {
            Ok(aps_frame) => {
                info!("Decoded APS frame: {aps_frame:?}");
            }
            Err(error) => {
                error!("Failed to parse APS frame from incoming message: {error}");
            }
        }
    }

    async fn handle_stack_status(&self, status: Result<ember::Status, u8>) {
        match match status {
            Ok(status) => status,
            Err(value) => {
                error!("Received invalid stack status: {value}");
                return;
            }
        } {
            ember::Status::NetworkUp => {
                self.outgoing
                    .send(Event::NetworkUp)
                    .await
                    .unwrap_or_else(|error| {
                        error!("Failed to send NetworkUp event: {error}");
                    });
            }
            ember::Status::NetworkDown => {
                self.outgoing
                    .send(Event::NetworkDown)
                    .await
                    .unwrap_or_else(|error| {
                        error!("Failed to send NetworkUp event: {error}");
                    });
            }
            ember::Status::NetworkOpened => {
                self.outgoing
                    .send(Event::NetworkOpened)
                    .await
                    .unwrap_or_else(|error| {
                        error!("Failed to send NetworkOpened event: {error}");
                    });
            }
            ember::Status::NetworkClosed => {
                self.outgoing
                    .send(Event::NetworkClosed)
                    .await
                    .unwrap_or_else(|error| {
                        error!("Failed to send NetworkOpened event: {error}");
                    });
            }
            other => {
                warn!("Received unhandled stack status: {other:?}");
            }
        }
    }

    async fn handle_trust_center_join(&self, trust_center_join: TrustCenterJoin) {
        match match trust_center_join.status() {
            Ok(status) => status,
            Err(value) => {
                error!("Received invalid trust center join status: {value}");
                return;
            }
        } {
            Update::StandardSecurityUnsecuredJoin => {
                self.outgoing
                    .send(Event::DeviceJoined {
                        ieee_address: trust_center_join.new_node_eui64(),
                        pan_id: trust_center_join.new_node_id(),
                    })
                    .await
                    .unwrap_or_else(|error| {
                        error!("Failed to send NetworkOpened event: {error}");
                    });
            }
            Update::StandardSecurityUnsecuredRejoin => {
                self.outgoing
                    .send(Event::DeviceRejoined {
                        ieee_address: trust_center_join.new_node_eui64(),
                        pan_id: trust_center_join.new_node_id(),
                        secured: false,
                    })
                    .await
                    .unwrap_or_else(|error| {
                        error!("Failed to send NetworkOpened event: {error}");
                    });
            }
            Update::StandardSecuritySecuredRejoin => {
                self.outgoing
                    .send(Event::DeviceRejoined {
                        ieee_address: trust_center_join.new_node_eui64(),
                        pan_id: trust_center_join.new_node_id(),
                        secured: true,
                    })
                    .await
                    .unwrap_or_else(|error| {
                        error!("Failed to send NetworkOpened event: {error}");
                    });
            }
            Update::DeviceLeft => {
                self.outgoing
                    .send(Event::DeviceLeft {
                        ieee_address: trust_center_join.new_node_eui64(),
                        pan_id: trust_center_join.new_node_id(),
                    })
                    .await
                    .unwrap_or_else(|error| {
                        error!("Failed to send NetworkOpened event: {error}");
                    });
            }
        }
    }
}
