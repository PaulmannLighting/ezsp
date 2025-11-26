//! Test `echo` command.

use ashv2::{BaudRate, FlowControl, SerialPort, open};
use clap::Parser;
use ezsp::uart::Uart;
use ezsp::{Callback, Utilities};
use log::{debug, error, info};
use tokio::sync::mpsc::channel;

const DEFAULT_VERSION: u8 = 8;
const DEFAULT_CHANNEL_SIZE: usize = 8;

#[derive(Debug, Parser)]
struct Args {
    #[arg(index = 1, help = "Path to the serial port")]
    tty: String,
    #[arg(index = 2, help = "Strings to echo")]
    texts: Vec<String>,
    #[arg(long, short, default_value_t = DEFAULT_VERSION, help = "EZSP version to negotiate")]
    version: u8,
    #[arg(long, short, help = "Keep listening for incoming messages")]
    keep_listening: bool,
    #[arg(long, short = 's', default_value_t = DEFAULT_CHANNEL_SIZE, help = "Channel size")]
    channel_size: usize,
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
    let (callbacks_tx, mut callbacks_rx) = channel::<Callback>(args.channel_size);

    tokio::spawn(async move {
        loop {
            if let Some(callback) = callbacks_rx.recv().await {
                debug!("Received callback: {callback:#?}");
            }
        }
    });

    let mut uart = Uart::new(serial_port, callbacks_tx, args.version, args.channel_size);

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
