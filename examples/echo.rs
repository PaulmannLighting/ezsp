//! Test version negotiation.

use ashv2::{open, BaudRate};
use clap::Parser;
use log::{debug, error, info};
use serialport::{FlowControl, SerialPort};

use ezsp::uart::Uart;
use ezsp::{Callback, Handler, Utilities};

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
    let mut uart = Uart::new(serial_port, StubHandler, args.version, 8);

    for text in args.texts {
        match uart.echo(text.bytes().collect()).await {
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

struct StubHandler;

impl Handler for StubHandler {
    fn handle(&mut self, callback: Callback) {
        debug!("Received callback: {callback:#?}");
    }
}
