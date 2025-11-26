//! Interactive command parsing.

use core::iter::once;
use core::str::FromStr;

use ashv2::SerialPort;
use clap::{Error, Parser};
use ezsp::uart::Uart;

use self::echo::echo;
use self::scan::scan;
use self::token_factory_reset::token_factory_reset;

mod echo;
mod scan;
mod token_factory_reset;

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
    TokenFactoryReset {
        #[arg(short = 'f', long, help = "Exclude the outgoing frame counter")]
        exclude_outgoing_frame_counter: bool,
        #[arg(short = 'b', long, help = "Exclude the boot counter")]
        exclude_boot_counter: bool,
    },
}

impl Command {
    pub async fn run<T>(self, uart: &mut Uart<T>)
    where
        T: SerialPort + 'static,
    {
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
            Self::TokenFactoryReset {
                exclude_outgoing_frame_counter,
                exclude_boot_counter,
            } => {
                token_factory_reset(uart, exclude_outgoing_frame_counter, exclude_boot_counter)
                    .await;
            }
        }
    }
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_parse_from(once("").chain(s.split_whitespace()))
    }
}
