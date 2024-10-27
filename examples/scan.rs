//! Test version negotiation.

use ashv2::{make_pair, open, BaudRate, Payload};
use clap::Parser;
use ezsp::ember::zigbee::Network;
use ezsp::ezsp::network::scan::Type;
use ezsp::uart::{Callbacks, Uart};
use ezsp::{parameters, Ezsp, Handler, Networking, MAX_FRAME_SIZE};
use log::{error, info, warn};
use serialport::{FlowControl, SerialPort};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread::spawn;

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
    let (cb_tx, cb_rx) = sync_channel(32);
    let (ash, transceiver) = make_pair::<MAX_FRAME_SIZE, _>(serial_port, 4, Some(cb_tx));
    let running = Arc::new(AtomicBool::new(true));
    let transceiver_running = running.clone();
    let transceiver_thread = spawn(|| transceiver.run(transceiver_running));
    let mut ezsp = Uart::new(ash);

    let callback_thread = spawn(move || handle_callbacks(&cb_rx, args.keep_listening));

    // Test version negotiation.
    match ezsp.negotiate_version(args.version).await {
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

    match ezsp
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

    callback_thread.join().expect("Callback thread panicked.");
    running.store(false, Relaxed);
    transceiver_thread
        .join()
        .expect("Transceiver thread panicked.");
}

fn handle_callbacks(frames: &Receiver<Payload>, keep_listening: bool) {
    for result in frames.iter().callbacks() {
        match result {
            Ok(handler) => match handler {
                Handler::Networking(parameters::networking::handler::Handler::NetworkFound(
                    network_found,
                )) => {
                    info!(
                        "Network found: last hop RSSI: {}, last hop LQI: {}",
                        network_found.last_hop_lqi(),
                        network_found.last_hop_lqi()
                    );
                    print_network(network_found.network_found());
                }
                Handler::Networking(parameters::networking::handler::Handler::ScanComplete(
                    scan_complete,
                )) => {
                    info!("Scan completed.");

                    if let Some(channel) = scan_complete.channel() {
                        error!("Scan failed on channel: {:#04X}", channel);
                    } else {
                        info!("Scan succeeded.");
                    }

                    if !keep_listening {
                        return;
                    }
                }
                other => {
                    warn!("Received unexpected handler: {other:?}");
                }
            },
            Err(error) => {
                error!("Error parsing handler: {error}");
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
