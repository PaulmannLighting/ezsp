//! Test version negotiation.

use ashv2::{make_pair, open, BaudRate};
use clap::Parser;
use ezsp::{Ashv2, Ezsp, SinkTable, Utilities};
use log::{error, info};
use serialport::{FlowControl, SerialPort};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread::spawn;

const TEST_TEXT: &str = "Rust rules! 🦀";

#[derive(Debug, Parser)]
struct Args {
    #[arg(index = 1, help = "Path to the serial port")]
    tty: String,
    #[arg(short, long, help = "EZSP version to negotiate")]
    version: u8,
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
    let (ash, transceiver) = make_pair::<4, _>(serial_port, None);
    let running = Arc::new(AtomicBool::new(true));
    let transceiver_thread = spawn(|| transceiver.run(running));
    let mut ezsp = Ashv2::new(ash);

    // Test version negotiation.
    match ezsp.negotiate_version(args.version).await {
        Ok(version) => {
            info!(
                "Negotiated protocol version: {:#04X}",
                version.protocol_version()
            );
            info!("Negotiated stack type: {:#04X}", version.stack_type());
            info!("Negotiated stack version: {:#06X}", version.stack_version());
        }
        Err(error) => {
            error!("Error negotiating version: {error}");
            return;
        }
    }

    // Test echo reply. Should be same as sent text.
    match ezsp.echo(TEST_TEXT.bytes().collect()).await {
        Ok(echo) => match String::from_utf8(echo.to_vec()) {
            Ok(echo) => {
                info!("Got echo: {echo}");
            }
            Err(error) => {
                error!("{error}");
            }
        },
        Err(error) => {
            error!("Error getting echo reply: {error}");
        }
    }

    // Test PRNG
    match ezsp.get_random_number().await {
        Ok(number) => {
            info!("Got random number: {number}");
        }
        Err(error) => {
            error!("Error getting random number: {error}");
        }
    }

    // Test XNCP
    match ezsp.get_xncp_info().await {
        Ok(info) => {
            info!("XNCP manufacturer ID: {}", info.manufacturer_id());
            info!("XNCP version number: {}", info.version_number());
        }
        Err(error) => {
            error!("Error getting XNCP info: {error}");
        }
    }

    // Test getting EUI64
    match ezsp.get_eui64().await {
        Ok(eui64) => {
            info!("EUI64: {eui64}");
        }
        Err(error) => {
            error!("Error getting EUI64: {error}");
        }
    }

    // Test getting node ID
    match ezsp.get_node_id().await {
        Ok(node_id) => {
            info!("Node ID: {node_id:#06X}");
        }
        Err(error) => {
            error!("Error getting node ID: {error}");
        }
    }

    // Test getting phy interface count
    match ezsp.get_phy_interface_count().await {
        Ok(phy_interfaces) => {
            info!("Phy interfaces: {phy_interfaces}");
        }
        Err(error) => {
            error!("Error getting phy interfaces: {error}");
        }
    }

    // Test getting true random entropy source
    match ezsp.get_true_random_entropy_source().await {
        Ok(entropy_source) => {
            info!("Entropy source: {entropy_source:?}");
        }
        Err(error) => {
            error!("Error getting entropy source: {error}");
        }
    }

    // Test getting number of sink table entries.
    match ezsp.number_of_active_entries().await {
        Ok(active_entries) => {
            info!("Sink table active entries: {active_entries:?}");
        }
        Err(error) => {
            error!("Error getting sink table active entries: {error}");
        }
    }

    if args.keep_listening {
        transceiver_thread
            .join()
            .expect("Transceiver thread panicked");
    }
}
