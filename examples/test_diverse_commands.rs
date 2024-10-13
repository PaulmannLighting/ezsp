//! Test version negotiation.

use ashv2::{make_pair, open, BaudRate};
use clap::Parser;
use ezsp::{Ashv2, Ezsp, Utilities};
use log::{error, info};
use serialport::{FlowControl, SerialPort};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread::spawn;

const TEST_TEXT: &str = "
וכשרוחות קרות יסערו בחוץ
אשלח בך אש חמה
";

#[derive(Debug, Parser)]
struct Args {
    #[arg(index = 1, help = "Path to the serial port")]
    tty: String,
    #[arg(short, long, help = "EZSP version to negotiate")]
    version: u8,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();

    match open(args.tty, BaudRate::RstCts, FlowControl::Software) {
        Ok(serial_port) => run(serial_port, args.version).await,
        Err(error) => error!("{error}"),
    }
}

async fn run(serial_port: impl SerialPort + Sized + 'static, version: u8) {
    let (ash, transceiver) = make_pair::<8, _>(serial_port, None);
    let running = Arc::new(AtomicBool::new(true));
    let _transceiver_thread = spawn(|| transceiver.run(running));
    let mut ezsp = Ashv2::new(ash);

    // Test version negotiation.
    match ezsp.negotiate_version(version).await {
        Ok(version) => {
            info!(
                "Negotiated protocol version: {:#04X}",
                version.protocol_version()
            );
            info!("Negotiated stack type: {:#04X}", version.stack_type());
            info!("Negotiated stack version: {:#06X}", version.stack_version());
        }
        Err(error) => {
            error!("{error}");
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
            error!("{error}");
        }
    }

    // Test PRNG
    match ezsp.get_random_number().await {
        Ok(number) => {
            info!("Got random number: {number}");
        }
        Err(error) => {
            error!("{error}");
        }
    }

    // Test XNCP
    match ezsp.get_xncp_info().await {
        Ok(info) => {
            info!("XNPC manufacturer ID: {}", info.manufacturer_id());
            info!("XNPC version number: {}", info.version_number());
        }
        Err(error) => {
            error!("{error}");
        }
    }

    // Test getting EUI64
    match ezsp.get_eui64().await {
        Ok(eui64) => {
            info!("EUI64: {eui64:#018X}");
        }
        Err(error) => {
            error!("{error}");
        }
    }

    // Test getting node ID
    match ezsp.get_node_id().await {
        Ok(node_id) => {
            info!("Node ID: {node_id:#06X}");
        }
        Err(error) => {
            error!("{error}");
        }
    }

    // Test getting phy interface count
    match ezsp.get_phy_interface_count().await {
        Ok(phy_interfaces) => {
            info!("Phy interfaces: {phy_interfaces}");
        }
        Err(error) => {
            error!("{error}");
        }
    }

    // Test getting true random entropy source
    match ezsp.get_true_random_entropy_source().await {
        Ok(entropy_source) => {
            info!("Entropy source: {entropy_source:?}");
        }
        Err(error) => {
            error!("{error}");
        }
    }
}
