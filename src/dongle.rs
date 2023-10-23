use crate::Protocol;
use serialport::SerialPort;
use stack::Stack;
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread::JoinHandle;
use std::time::SystemTime;

pub mod stack;

// XXX: Mockups
pub trait BootloaderHandler {}
pub trait ZigBeeTransportReceive {}

#[derive(Debug)]
pub struct ZigBeeKey;
#[derive(Debug)]
pub struct NetworkParameters;
#[derive(Debug)]
pub struct IeeeAddress;
#[derive(Debug)]
pub struct DeviceType;
#[derive(Debug)]
pub struct VersionResponse;
#[derive(Debug)]
pub struct NetworkState;
#[derive(Debug)]
pub struct ProfileId;
#[derive(Debug)]
pub struct DeviceId;
#[derive(Debug)]
pub struct EmberMfglibListener;
#[derive(Debug)]
pub struct EmberNcpResetProvider;
#[derive(Debug)]
pub struct Concentrator;

#[derive(Debug)]
pub struct Dongle<B, T, P, S>
where
    B: BootloaderHandler,
    T: ZigBeeTransportReceive,
    P: Protocol,
    S: SerialPort,
{
    serial_port: S,
    bootload_handler: B,
    stack: Stack,
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
