//! Test version negotiation.

use ashv2::{make_pair, open, BaudRate, CallbacksFramed, Payload};
use clap::Parser;
use ezsp::ezsp::network::scan::Type;
use ezsp::{Ashv2, Codec, Ezsp, Networking, EZSP_MAX_FRAME_SIZE};
use futures::StreamExt;
use log::{error, info};
use serialport::{FlowControl, SerialPort};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::Arc;
use std::task::Waker;
use std::thread::spawn;
use tokio_util::codec::Framed;

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
    let (cb_waker_tx, cb_waker_rx) = sync_channel(32);
    let (ash, transceiver) =
        make_pair::<EZSP_MAX_FRAME_SIZE, _>(serial_port, 4, Some((cb_tx, cb_waker_rx)));
    let running = Arc::new(AtomicBool::new(true));
    let transceiver_thread = spawn(|| transceiver.run(running));
    let mut ezsp = Ashv2::new(ash);

    tokio::task::spawn(handle_callbacks(cb_waker_tx, cb_rx));

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

    transceiver_thread
        .join()
        .expect("Transceiver thread panicked.");
}

async fn handle_callbacks(waker: SyncSender<Waker>, callbacks: Receiver<Payload>) {
    let mut callbacks = Framed::new(CallbacksFramed::new(waker, callbacks), Codec::default());

    loop {
        if let Some(result) = callbacks.next().await {
            match result {
                Ok(callback) => {
                    info!("Received callback: {:?}", callback);
                }
                Err(error) => {
                    error!("Error receiving callback: {error}");
                    continue;
                }
            }
        };
    }
}
