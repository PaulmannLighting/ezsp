use crate::config_id::ConfigId;
use crate::policy_id::PolicyId;
use serialport::SerialPort;
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread::JoinHandle;
use std::time::SystemTime;

pub struct Dongle<H, T, P>
where
    H: BootloaderHandler,
    T: ZigBeeTransportReceive,
    P: SerialProtocol,
{
    serial_port: SerialPort,
    bootload_handler: H,
    stack_config: HashMap<ConfigId, u8>,
    stack_policy: HashMap<PolicyId, DecisionId>,
    zigbee_transport_receive: T,
    zigbee_key: ZigBeeKey,
    network_key: ZigBeeKey,
    network_parameters: NetworkParameters,
    ieee_address: IeeeAddress,
    network_address: u32,
    device_type: DeviceType,
    serial_protocol: P,
    version: VersionResponse,
    version_string: String,
    network_state: NetworkState,
    initialized: bool,
    forward_loopback_messages: bool,
    default_profile: ProfileId,
    default_device: DeviceId,
    is_configured: Mutex<bool>,
    executor_service: JoinHandle<()>, // ScheduledFuture<?>
    poll_rate: u16,
    last_command_sent: SystemTime,
    manufacturing_library_listener: EmberMfglibListener, // ?
    reset_provider: EmberNcpResetProvider,               // ?
    input_clusters: Vec<u8>,
    output_clusters: Vec<u8>,
    fragmentation_aps_counter: HashMap<u8, u8>,
    concentrator_type: Concentrator,
}
