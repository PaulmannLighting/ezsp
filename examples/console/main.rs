//! Test version negotiation.

use ashv2::{open, BaudRate};
use clap::Parser;
use log::error;
use serialport::{FlowControl, SerialPort};
use std::io::{stdin, stdout, Write};
use std::str::FromStr;

use ezsp::uart::Uart;
use ezsp::{Ezsp, Utilities};

use args::Args;
use command::Command;
use handler::Handler;

mod args;
mod command;
mod handler;
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

async fn run(serial_port: impl SerialPort + Sized + 'static, args: Args) {
    let mut uart = Uart::new(serial_port, Handler, args.version, 8);

    if let Err(error) = uart.init().await {
        error!("{error}");
        return;
    }

    let mut lines = stdin().lines().map_while(Result::ok);

    loop {
        prompt("EZSP> ").expect("STDOUT should be available.");

        let Some(line) = lines.next() else {
            break;
        };

        if line.is_empty() {
            match uart.nop().await {
                Ok(()) => {
                    continue;
                }
                Err(error) => {
                    println!("{error}");
                    break;
                }
            }
        }

        match Command::from_str(&line) {
            Ok(command) => {
                command.run(&mut uart).await;
            }
            Err(error) => {
                println!("{error}");
            }
        }
    }
}

fn prompt(message: impl AsRef<[u8]>) -> std::io::Result<()> {
    stdout().write_all(message.as_ref())?;
    stdout().flush()
}
