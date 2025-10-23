use clap::Parser;

const DEFAULT_VERSION: u8 = 8;
const DEFAULT_CHANNEL_MASK: u32 = 0x0000_0000;
const DEFAULT_SCAN_DURATION: u8 = 0x00;
const DEFAULT_CHANNEL_SIZE: usize = 8;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(index = 1, help = "Path to the serial port")]
    pub(crate) tty: String,
    #[arg(long, short, default_value_t = DEFAULT_VERSION, help = "EZSP version to negotiate")]
    pub(crate) version: u8,
    #[arg(
        long,
        short,
        default_value_t = DEFAULT_CHANNEL_MASK,
        help = "Channel mask for scan command"
    )]
    pub(crate) channel_mask: u32,
    #[arg(
        long,
        short,
        default_value_t = DEFAULT_SCAN_DURATION,
        help = "Duration for scan command"
    )]
    pub(crate) scan_duration: u8,
    #[arg(long, short = 's', default_value_t = DEFAULT_CHANNEL_SIZE, help = "Channel size")]
    pub(crate) channel_size: usize,
}
