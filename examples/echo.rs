//! Test version negotiation.

use ashv2::{make_pair, open, BaudRate};
use clap::Parser;
use ezsp::{Ashv2, Ezsp, Utilities};
use log::{error, info};
use serialport::{FlowControl, SerialPort};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread::spawn;

const DEFAULT_VERSION: u8 = 8;

#[derive(Debug, Parser)]
struct Args {
    #[arg(index = 1, help = "Path to the serial port")]
    tty: String,
    #[arg(index = 2, help = "Strings to echo")]
    texts: Vec<String>,
    #[arg(short, long, default_value_t = DEFAULT_VERSION, help = "EZSP version to negotiate")]
    version: u8,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();

    match open(args.tty, BaudRate::RstCts, FlowControl::Software) {
        Ok(serial_port) => run(serial_port, &args.texts, args.version).await,
        Err(error) => error!("{error}"),
    }
}

async fn run(serial_port: impl SerialPort + Sized + 'static, texts: &[String], version: u8) {
    let (ash, transceiver) = make_pair::<8, _>(serial_port, None);
    let running = Arc::new(AtomicBool::new(true));
    let _transceiver_thread = spawn(|| transceiver.run(running));
    let mut ezsp = Ashv2::new(ash);

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

    for text in texts {
        match ezsp.echo(text.bytes().collect()).await {
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
    }
}
