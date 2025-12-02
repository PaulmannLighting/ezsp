use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;

use log::{debug, error, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::parameters::binding::handler::Handler as BindingEvent;
use crate::parameters::bootloader::handler::Handler as BootloaderEvent;
use crate::parameters::cbke::handler::Handler as CbkeEvent;
use crate::parameters::green_power::handler::{
    Handler as GreenPowerEvent, Payload as GreenPowerMessage,
};
use crate::parameters::messaging::handler::Handler as MessagingEvent;
use crate::parameters::mfglib::handler::Handler as MfglibEvent;
use crate::parameters::networking::handler::Handler as NetworkingEvent;
use crate::parameters::security::handler::Handler as SecurityEvent;
use crate::parameters::trust_center::handler::Handler as TrustCenterEvent;
use crate::parameters::utilities::handler::Handler as UtilitiesEvent;
use crate::parameters::zll::handler::Handler as ZllEvent;
use crate::zigbee::network_manager::zigbee_message::ZigbeeMessage;
use crate::{Callback, ember};

/// EZSP event handler.
pub struct EventHandler {
    events: Receiver<Callback>,
    zigbee_messages: Sender<ZigbeeMessage>,
    network_up: Arc<AtomicBool>,
    network_open: Arc<AtomicBool>,
}

impl EventHandler {
    /// Create a new `EventHandler`.
    pub const fn new(
        events: Receiver<Callback>,
        zigbee_messages: Sender<ZigbeeMessage>,
        network_up: Arc<AtomicBool>,
        network_open: Arc<AtomicBool>,
    ) -> Self {
        Self {
            events,
            zigbee_messages,
            network_up,
            network_open,
        }
    }

    /// Run the event handler.
    pub async fn run(mut self) {
        while let Some(event) = self.events.recv().await {
            self.handle_event(event).await;
        }

        debug!("EZSP event handler has stopped receiving events.");
    }

    async fn handle_event(&self, event: Callback) {
        match event {
            Callback::Binding(event) => {
                self.handle_binding_event(event);
            }
            Callback::Bootloader(event) => {
                self.handle_bootloader_event(event);
            }
            Callback::Cbke(event) => {
                self.handle_cbke_event(event);
            }
            Callback::GreenPower(event) => {
                self.handle_green_power_event(event);
            }
            Callback::Messaging(event) => {
                self.handle_messaging_event(event).await;
            }
            Callback::MfgLib(event) => {
                self.handle_mfglib_event(event);
            }
            Callback::Networking(event) => {
                self.handle_networking_event(event).await;
            }
            Callback::Security(event) => {
                self.handle_security_event(event);
            }
            Callback::TrustCenter(event) => {
                self.handle_trust_center_event(event).await;
            }
            Callback::Utilities(event) => {
                self.handle_utilities_event(event);
            }
            Callback::Zll(event) => {
                self.handle_zll_event(event);
            }
        }
    }

    fn handle_binding_event(&self, event: BindingEvent) {
        match event {
            BindingEvent::RemoteDeleteBinding(event) => match u8::try_from(event) {
                Ok(index) => {
                    debug!("Deleting binding at index: {index}");
                }
                Err(error) => {
                    debug!("Failed to remove binding: {error}");
                }
            },
            BindingEvent::RemoteSetBinding(event) => match (*event).try_into() {
                Ok((index, entry)) => {
                    debug!("Setting binding at index {index}: {entry:?}");
                }
                Err(error) => {
                    debug!("Failed to set binding: {error}");
                }
            },
        }
    }

    fn handle_bootloader_event(&self, event: BootloaderEvent) {
        match event {
            BootloaderEvent::BootloadTransmitComplete(event) => {
                debug!("Bootloader transmit complete: {event:?}");
            }
            BootloaderEvent::IncomingBootloadMessage(event) => {
                debug!("Incoming bootload message: {event:?}");
            }
        }
    }

    fn handle_cbke_event(&self, event: CbkeEvent) {
        match event {
            CbkeEvent::CalculateSmacs(event) => {
                debug!("CBKE CalculateSmacs event: {event:?}");
            }
            CbkeEvent::CalculateSmacs283k1(event) => {
                debug!("CBKE CalculateSmacs283k1 event: {event:?}");
            }
            CbkeEvent::DsaSign(event) => {
                debug!("CBKE DsaSign event: {event:?}");
            }
            CbkeEvent::DsaVerify(event) => {
                debug!("CBKE DsaVerify event: {event:?}");
            }
            CbkeEvent::GenerateCbkeKeys(event) => {
                debug!("CBKE GenerateCbkeKeys event: {event:?}");
            }
            CbkeEvent::GenerateCbkeKeys283k1(event) => {
                debug!("CBKE GenerateCbkeKeys283k1 event: {event:?}");
            }
        }
    }

    fn handle_green_power_event(&self, event: GreenPowerEvent) {
        match event {
            GreenPowerEvent::IncomingMessage(incoming_message) => {
                debug!("Green Power incoming message: {incoming_message:?}");

                match GreenPowerMessage::try_from(incoming_message) {
                    Ok(payload) => {
                        debug!("Forwarding Green Power payload: {payload:?}");
                    }
                    Err(error) => {
                        debug!("Failed to process Green Power incoming message: {error}");
                    }
                }
            }
            GreenPowerEvent::Sent(sent) => {
                debug!("Green Power sent message: {sent:?}");
            }
        }
    }

    async fn handle_messaging_event(&self, event: MessagingEvent) {
        match event {
            MessagingEvent::IdConflict(id_conflict) => {
                warn!("ID conflict: {:#06X}", id_conflict.id());
            }
            MessagingEvent::IncomingManyToOneRouteRequest(route_request) => {
                debug!("Incoming many-to-one route request: {route_request:?}");
            }
            MessagingEvent::IncomingMessage(message) => {
                debug!("Incoming message: {message:?}");

                if let Err(error) = self.zigbee_messages.send(message.into()).await {
                    error!("Failed to forward incoming message: {error}");
                }
            }
            MessagingEvent::IncomingNetworkStatus(network_status) => {
                warn!(
                    "Incoming network status for {:#06X}: {:#04X}",
                    network_status.target(),
                    network_status.error_code()
                );
            }
            MessagingEvent::IncomingRouteError(route_error) => match route_error.status() {
                Ok(status) => {
                    warn!(
                        "Incoming route error for {}: {status}",
                        route_error.target(),
                    );
                }
                Err(error) => error!(
                    "Invalid route error status for {}: {error}",
                    route_error.target()
                ),
            },
            MessagingEvent::IncomingRouteRecord(route_record) => {
                debug!("Incoming route record: {route_record:?}");
            }
            MessagingEvent::IncomingSenderEui64(sender_eui64) => {
                debug!("Incoming sender EUI64: {sender_eui64:?}");
            }
            MessagingEvent::MacFilterMatchMessage(mac_filter_match) => {
                debug!("MAC filter match message: {mac_filter_match:?}");
            }
            MessagingEvent::MacPassthroughMessage(mac_passthrough) => {
                debug!("MAC passthrough message: {mac_passthrough:?}");
            }
            MessagingEvent::MessageSent(message_sent) => {
                debug!("Message sent: {message_sent:?}");
            }
            MessagingEvent::Poll(poll) => {
                debug!("Poll event: {poll:?}");
            }
            MessagingEvent::PollComplete(poll_complete) => {
                debug!("Poll complete event: {poll_complete:?}");
            }
            MessagingEvent::RawTransmitComplete(raw_transmit_complete) => {
                debug!("Raw transmit complete: {raw_transmit_complete:?}");
            }
        }
    }

    fn handle_mfglib_event(&self, event: MfglibEvent) {
        match event {
            MfglibEvent::Rx(event) => {
                debug!("Mfglib RX event: {event:?}");
            }
        }
    }

    async fn handle_networking_event(&self, event: NetworkingEvent) {
        match event {
            NetworkingEvent::ChildJoin(child_join) => {
                debug!("Child join event: {child_join:?}");

                if let Err(error) = self.zigbee_messages.send(child_join.into()).await {
                    error!("Failed to forward child join message: {error}");
                }
            }
            NetworkingEvent::DutyCycle(duty_cycle) => {
                debug!("Duty cycle event: {duty_cycle:?}");
            }
            NetworkingEvent::EnergyScanResult(energy_scan_result) => {
                debug!("Energy scan result event: {energy_scan_result:?}");
            }
            NetworkingEvent::NetworkFound(network_found) => {
                debug!("Network found event: {network_found:?}");
            }
            NetworkingEvent::ScanComplete(scan_complete) => {
                debug!("Scan complete event: {scan_complete:?}");
            }
            NetworkingEvent::StackStatus(stack_status) => match stack_status.result() {
                Ok(status) => match status {
                    ember::Status::NetworkUp => {
                        debug!("Network up");
                        self.network_up.store(true, SeqCst);
                    }
                    ember::Status::NetworkDown => {
                        debug!("Network down");
                        self.network_up.store(false, SeqCst);
                        self.network_open.store(false, SeqCst);
                    }
                    ember::Status::NetworkOpened => {
                        debug!("Network opened (joinable)");
                        self.network_open.store(true, SeqCst);
                    }
                    ember::Status::NetworkClosed => {
                        debug!("Network closed (not joinable)");
                        self.network_open.store(false, SeqCst);
                    }
                    other => {
                        debug!("Other stack status: {other}");
                    }
                },
                Err(error) => {
                    warn!("Received invalid stack status: {error}");
                }
            },
            NetworkingEvent::UnusedPanIdFound(unused_pan_id_found) => {
                debug!("Unused PAN ID found event: {unused_pan_id_found:?}");
            }
        }
    }

    fn handle_security_event(&self, event: SecurityEvent) {
        match event {
            SecurityEvent::SwitchNetworkKey(switch_network_key) => {
                debug!("Incoming TC link key event: {switch_network_key:?}");
            }
            SecurityEvent::ZigbeeKeyEstablishment(zigbee_key_establishment) => {
                debug!("MAC key update event: {zigbee_key_establishment:?}");
            }
        }
    }

    async fn handle_trust_center_event(&self, event: TrustCenterEvent) {
        match event {
            TrustCenterEvent::TrustCenterJoin(join) => {
                trace!("Trust center join event: {join:?}");

                match ZigbeeMessage::try_from(join) {
                    Ok(message) => {
                        if let Err(error) = self.zigbee_messages.send(message).await {
                            error!("Failed to forward trust center join message: {error}");
                        }
                    }
                    Err(error) => {
                        error!("Failed to process trust center join event: {error}");
                    }
                }
            }
        }
    }

    fn handle_utilities_event(&self, event: UtilitiesEvent) {
        match event {
            UtilitiesEvent::CounterRollover(rollover) => {
                debug!("Counter rollover event: {rollover:?}");
            }
            UtilitiesEvent::CustomFrame(custom_frame) => {
                debug!("Custom frame event: {custom_frame:?}");
            }
            UtilitiesEvent::StackTokenChanged(stack_token_changed) => {
                debug!("Stack token changed event: {stack_token_changed:?}");
            }
            UtilitiesEvent::Timer(timer) => {
                debug!("Timer event: {timer:?}");
            }
        }
    }

    fn handle_zll_event(&self, event: ZllEvent) {
        match event {
            ZllEvent::AddressAssignment(address_assignment) => {
                debug!("ZLL Address Assignment event: {address_assignment:?}");
            }
            ZllEvent::NetworkFound(network_found) => {
                debug!("ZLL Network Found event: {network_found:?}");
            }
            ZllEvent::ScanComplete(scan_complete) => {
                debug!("ZLL Scan Complete event: {scan_complete:?}");
            }
            ZllEvent::TouchLinkTarget(touch_link_target) => {
                debug!("ZLL Touch Link Target event: {touch_link_target:?}");
            }
        }
    }
}
