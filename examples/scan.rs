//! Test version negotiation.

use ashv2::{open, BaudRate};
use clap::Parser;
use ezsp::ember::zigbee::Network;
use ezsp::ezsp::network::scan::Type;
use ezsp::uart::Uart;
use ezsp::{parameters, Callback, Ezsp, Handler, Networking, MAX_FRAME_SIZE};
use log::{error, info, warn};
use serialport::{FlowControl, SerialPort};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Parser)]
struct Args {
    #[arg(index = 1, help = "Path to the serial port")]
    tty: String,
    #[arg(short, long, help = "EZSP version to negotiate")]
    version: u8,
    #[arg(
        short,
        long,
        default_value_t = 0x0000_0000,
        help = "Channel mask for scan command"
    )]
    channel_mask: u32,
    #[arg(
        short,
        long,
        default_value_t = 0x00,
        help = "Duration for scan command"
    )]
    scan_duration: u8,
    #[arg(short, long, help = "Keep listening for incoming messages")]
    keep_listening: bool,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();

    match open(args.tty.clone(), BaudRate::RstCts, FlowControl::Software) {
        Ok(serial_port) => run(serial_port, args).await,
        Err(error) => error!("{error}"),
    }
}

async fn run(serial_port: impl SerialPort + Sized + 'static, args: Args) {
    let mut uart = Uart::new(serial_port, NetworkScanHandler, 8);

    // Test version negotiation.
    match uart.negotiate_version(args.version).await {
        Ok(version) => {
            info!(
                "Negotiated protocol version: {:#04X}",
                version.protocol_version()
            );
            info!("Negotiated stack type: {:#04X}", version.stack_type());
            info!("Negotiated stack version: {}", version.stack_version());
        }
        Err(error) => {
            error!("Error negotiating version: {error}");
            return;
        }
    }

    match uart
        .start_scan(Type::ActiveScan, args.channel_mask, args.scan_duration)
        .await
    {
        Ok(()) => {
            info!("Started a scan");
        }
        Err(error) => {
            error!("Error starting scan: {error}");
        }
    }

    sleep(Duration::from_secs(15)).await;
}

struct NetworkScanHandler;

impl Handler for NetworkScanHandler {
    fn handle(&mut self, callback: Callback) {
        info!("Handling callback.");
        match callback {
            Callback::Networking(parameters::networking::handler::Handler::NetworkFound(
                network_found,
            )) => {
                info!(
                    "Network found: last hop RSSI: {}, last hop LQI: {}",
                    network_found.last_hop_lqi(),
                    network_found.last_hop_lqi()
                );
                print_network(network_found.network_found());
            }
            Callback::Networking(parameters::networking::handler::Handler::ScanComplete(
                scan_complete,
            )) => {
                info!("Scan completed.");

                if let Some(channel) = scan_complete.channel() {
                    error!("Scan failed on channel: {:#04X}", channel);
                } else {
                    info!("Scan succeeded.");
                }
            }
            other => {
                warn!("Received unexpected handler: {other:?}");
            }
        }
    }
}

fn print_network(network: &Network) {
    info!("  * channel: {:#04X}", network.channel());
    info!("  * PAN ID: {:#06X}", network.pan_id());
    info!("  * Extended PAN ID: {}", network.extended_pan_id());
    info!("  * Allowing join: {}", network.allowing_join());
    info!("  * Stack profile: {}", network.stack_profile());
    info!("  * Update ID: {}", network.nwk_update_id());
}
