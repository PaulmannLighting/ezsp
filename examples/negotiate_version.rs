//! Test version negotiation.

use ashv2::{open, AshFramed, BaudRate, Transceiver};
use clap::Parser;
use ezsp::{Ashv2, Ezsp, Utilities};
use log::error;
use serialport::{FlowControl, SerialPort};
use std::io::Read;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::sync_channel;
use std::sync::Arc;
use std::thread::spawn;

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
    let (sender, receiver) = sync_channel(32);
    let transceiver = Transceiver::new(serial_port, receiver, None);
    let running = Arc::new(AtomicBool::new(true));
    let _transceiver_thread = spawn(|| transceiver.run(running));

    let mut ezsp = Ashv2::new(AshFramed::<2>::new(sender));

    // Test version negotiation.
    match ezsp.negotiate_version(version).await {
        Ok(version) => {
            println!("Negotiated version: {version:#06X?}");
        }
        Err(error) => {
            error!("{error}");
        }
    }

    // Test echo reply. Should be same as sent text.
    let text = "Hello, world!";

    match ezsp.echo(text.bytes().collect()).await {
        Ok(echo) => match String::from_utf8(echo.to_vec()) {
            Ok(echo) => {
                println!("Got echo: {echo}");
            }
            Err(error) => {
                error!("{error}");
            }
        },
        Err(error) => {
            error!("{error}");
        }
    }
}
