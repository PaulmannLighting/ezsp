//! Test `echo` command.

use ashv2::{BaudRate, open};
use clap::Parser;
use ezsp::uart::Uart;
use ezsp::{Callback, Utilities};
use log::{debug, error, info};
use serialport::{FlowControl, SerialPort};
use tokio::sync::mpsc::channel;

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

async fn run<S>(serial_port: S, args: Args)
where
    S: SerialPort + 'static,
{
    let (callbacks_tx, mut callbacks_rx) = channel::<Callback>(8);

    tokio::spawn(async move {
        loop {
            if let Some(callback) = callbacks_rx.recv().await {
                debug!("Received callback: {callback:#?}");
            }
        }
    });

    let mut uart = Uart::new(serial_port, callbacks_tx, args.version, 8);

    for text in args.texts {
        match uart.echo(text.bytes().collect()).await {
            Ok(echo) => match String::from_utf8(echo.to_vec()) {
                Ok(echo) => {
                    info!("Got echo: {echo}");
                    assert_eq!(echo, text);
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
