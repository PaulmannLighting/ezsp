//! Test `scan` command.

use ashv2::{BaudRate, FlowControl, SerialPort, open};
use clap::Parser;
use ezsp::ezsp::network::scan::Type;
use ezsp::uart::Uart;
use ezsp::{Callback, Networking};
use log::{error, info};
use tokio::sync::mpsc::channel;

use self::args::Args;
use self::print_echo::PrintEcho;
use self::utils::handle_callback;

mod args;
mod print_echo;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();

    match open(args.tty.clone(), BaudRate::RstCts, FlowControl::Software) {
        Ok(serial_port) => run(serial_port, args).await,
        Err(error) => error!("{error}"),
    }
}

async fn run<S>(serial_port: S, args: Args)
where
    S: SerialPort + 'static,
{
    let (callbacks_tx, mut callbacks_rx) = channel::<Callback>(args.channel_size);

    tokio::spawn(async move {
        loop {
            if let Some(callback) = callbacks_rx.recv().await {
                handle_callback(callback);
            }
        }
    });

    let mut uart = Uart::new(serial_port, callbacks_tx, args.version, args.channel_size);

    uart.print_echo("About to start a scan.").await;

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

    uart.print_echo("I just performed a scan.").await;
}
