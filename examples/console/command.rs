//! Interactive command parsing.

use clap::{Error, Parser};
use ezsp::uart::Uart;
use std::iter::once;
use std::str::FromStr;

use echo::echo;
use scan::scan;

mod echo;
mod scan;

/// Available subcommands.
#[derive(Debug, Parser)]
pub enum Command {
    Echo {
        #[arg(help = "Message to send")]
        message: Vec<String>,
    },
    Scan {
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
    },
}

impl Command {
    pub async fn run(self, uart: &mut Uart) {
        match self {
            Self::Echo { message } => {
                echo(uart, message.join(" ")).await;
            }
            Self::Scan {
                channel_mask,
                scan_duration,
            } => {
                scan(uart, channel_mask, scan_duration).await;
            }
        }
    }
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_parse_from(once("EZSP interactive shell").chain(s.split_whitespace()))
    }
}
