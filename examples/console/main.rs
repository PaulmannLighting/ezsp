//! Interactive console.

use std::io::{Write, stdin, stdout};
use std::str::FromStr;

use ashv2::{BaudRate, open};
use clap::Parser;
use log::error;
use serialport::{FlowControl, SerialPort};
use tokio::sync::mpsc::channel;

use ezsp::uart::Uart;
use ezsp::{Callback, Ezsp, Utilities};

use crate::handler::handle_callback;
use args::Args;
use command::Command;

mod args;
mod command;
mod handler;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();

    let (callbacks_tx, mut callbacks_rx) = channel::<Callback>(8);

    tokio::spawn(async move {
        loop {
            if let Some(callback) = callbacks_rx.recv().await {
                handle_callback(callback);
            }
        }
    });

    match open(args.tty.clone(), BaudRate::RstCts, FlowControl::Software) {
        Ok(serial_port) => run(Uart::new(serial_port, callbacks_tx, args.version, 8)).await,
        Err(error) => error!("{error}"),
    }
}

#[allow(clippy::future_not_send)]
async fn run<T>(mut uart: Uart<T>)
where
    T: SerialPort + 'static,
{
    if let Err(error) = uart.init().await {
        error!("{error}");
        return;
    }

    let mut lines = stdin().lines().map_while(Result::ok);

    loop {
        prompt("EZSP> ").expect("STDOUT should be available. This is a bug.");

        let Some(line) = lines.next() else {
            drop(lines);
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

/// Print the command line prompt.
fn prompt(message: impl AsRef<[u8]>) -> std::io::Result<()> {
    stdout().write_all(message.as_ref())?;
    stdout().flush()
}
