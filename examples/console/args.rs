//! Command line arguments parsing.

use clap::Parser;

/// Default `EZSP` version to negotiate.
const DEFAULT_VERSION: u8 = 8;

/// Command line arguments.
#[derive(Debug, Parser)]
pub struct Args {
    #[arg(index = 1, help = "Path to the serial port")]
    pub(crate) tty: String,
    #[arg(short, long, default_value_t = DEFAULT_VERSION, help = "EZSP version to negotiate")]
    pub(crate) version: u8,
}
