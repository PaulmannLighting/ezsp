//! Test version negotiation.

use ashv2::{open, AshFramed, BaudRate, Transceiver};
use clap::Parser;
use ezsp::ashv2::Ashv2;
use ezsp::{Ezsp, Utilities, MAX_FRAME_SIZE};
use log::{error, info};
use serialport::{FlowControl, SerialPort};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::sync_channel;
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
    //let (ash, transceiver) = make_pair::<MAX_FRAME_SIZE, _>(serial_port, 8, None);
    let (req_tx, req_rx) = sync_channel(8);
    let (waker_tx, waker_rx) = sync_channel(8);
    let ash = AshFramed::<MAX_FRAME_SIZE>::new(req_tx, waker_tx, 8);
    let transceiver = Transceiver::<_, 2>::new(serial_port, req_rx, waker_rx, None);
    let running = Arc::new(AtomicBool::new(true));
    let transceiver_thread = spawn(|| transceiver.run(running));
    let mut ezsp = Ashv2::new(ash);

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
            error!("{error}");
        }
    }

    for text in args.texts {
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

    if args.keep_listening {
        transceiver_thread
            .join()
            .expect("Transceiver thread panicked");
    }
}
