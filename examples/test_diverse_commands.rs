//! Test version negotiation.

use ashv2::{make_pair, open, BaudRate, HexSlice};
use clap::Parser;
use ezsp::ember::{CertificateData, PublicKeyData};
use ezsp::ezsp::network::scan::Type;
use ezsp::ezsp::value::Id;
use ezsp::{
    Ashv2, CertificateBasedKeyExchange, Configuration, Ezsp, Networking, ProxyTable, Security,
    SinkTable, Utilities, EZSP_MAX_FRAME_SIZE,
};
use le_stream::ToLeStream;
use log::{error, info};
use serialport::{FlowControl, SerialPort};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread::spawn;

const TEST_TEXT: &str = "Rust rules! 🦀";

#[derive(Debug, Parser)]
struct Args {
    #[arg(index = 1, help = "Path to the serial port")]
    tty: String,
    #[arg(short, long, help = "EZSP version to negotiate")]
    version: u8,
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

#[allow(clippy::too_many_lines)]
async fn run(serial_port: impl SerialPort + Sized + 'static, args: Args) {
    let (ash, transceiver) = make_pair::<EZSP_MAX_FRAME_SIZE, _>(serial_port, 4, None);
    let running = Arc::new(AtomicBool::new(true));
    let transceiver_thread = spawn(|| transceiver.run(running));
    let mut ezsp = Ashv2::new(ash);

    // Test version negotiation.
    match ezsp.negotiate_version(args.version).await {
        Ok(version) => {
            info!(
                "Negotiated protocol version: {:#04X}",
                version.protocol_version()
            );
            info!("Negotiated stack type: {:#04X}", version.stack_type());
            info!("Negotiated stack version: {:#06X}", version.stack_version());
        }
        Err(error) => {
            error!("Error negotiating version: {error}");
            return;
        }
    }

    // Test echo reply. Should be same as sent text.
    match ezsp.echo(TEST_TEXT.bytes().collect()).await {
        Ok(echo) => match String::from_utf8(echo.to_vec()) {
            Ok(echo) => {
                info!("Got echo: {echo}");
            }
            Err(error) => {
                error!("{error}");
            }
        },
        Err(error) => {
            error!("Error getting echo reply: {error}");
        }
    }

    // Test PRNG
    match ezsp.get_random_number().await {
        Ok(number) => {
            info!("Got random number: {number}");
        }
        Err(error) => {
            error!("Error getting random number: {error}");
        }
    }

    // Test XNCP
    match ezsp.get_xncp_info().await {
        Ok(info) => {
            info!("XNCP manufacturer ID: {}", info.manufacturer_id());
            info!("XNCP version number: {}", info.version_number());
        }
        Err(error) => {
            error!("Error getting XNCP info: {error}");
        }
    }

    // Test getting EUI64
    match ezsp.get_eui64().await {
        Ok(eui64) => {
            info!("EUI64: {eui64}");
        }
        Err(error) => {
            error!("Error getting EUI64: {error}");
        }
    }

    // Test getting node ID
    match ezsp.get_node_id().await {
        Ok(node_id) => {
            info!("Node ID: {node_id:#06X}");
        }
        Err(error) => {
            error!("Error getting node ID: {error}");
        }
    }

    // Test getting phy interface count
    match ezsp.get_phy_interface_count().await {
        Ok(phy_interfaces) => {
            info!("Phy interfaces: {phy_interfaces}");
        }
        Err(error) => {
            error!("Error getting phy interfaces: {error}");
        }
    }

    // Test getting true random entropy source
    match ezsp.get_true_random_entropy_source().await {
        Ok(entropy_source) => {
            info!("Entropy source: {entropy_source:?}");
        }
        Err(error) => {
            error!("Error getting entropy source: {error}");
        }
    }

    // Test getting number of sink table entries.
    match ezsp.number_of_active_entries().await {
        Ok(active_entries) => {
            info!("Sink table active entries: {active_entries:?}");
        }
        Err(error) => {
            error!("Error getting sink table active entries: {error}");
        }
    }

    get_value_ids(&mut ezsp).await;

    // Test exporting the transient key by index.
    match ezsp.export_transient_key_by_index(0).await {
        Ok(payload) => {
            info!("Transient key by index: {payload:?}");
        }
        Err(error) => {
            error!("Error getting transient key by index: {error}");
        }
    }

    // Test getting GP proxy table entry.
    match ProxyTable::get_entry(&mut ezsp, 0).await {
        Ok(entry) => {
            info!("GP proxy table entry: {entry:#04X?}");
            info!(
                "GP proxy table entry size: {}",
                entry.to_le_stream().count()
            );
        }
        Err(error) => {
            error!("Error getting GP proxy table entry: {error}");
        }
    }

    // Test getting Certificate 283k1 data.
    match ezsp.get_certificate283k1().await {
        Ok(certificate) => {
            info!("Certificate 283k1 data: {certificate:#04X?}");
            info!(
                "Certificate 283k1 data size: {}",
                certificate.to_le_stream().count()
            );
        }
        Err(error) => {
            error!("Error getting certificate 283k1 data: {error}");
        }
    }

    // Test getting Certificate data.
    match ezsp.get_certificate().await {
        Ok(certificate) => {
            info!("Certificate data: {certificate:#04X?}");
            info!(
                "Certificate data size: {}",
                certificate.to_le_stream().count()
            );
        }
        Err(error) => {
            error!("Error getting certificate data: {error}");
        }
    }

    // Test getting duty cycle limits.
    match ezsp.get_duty_cycle_limits().await {
        Ok(limits) => {
            info!("Duty cycle limits: {limits:#04X?}");
            info!("Duty cycle limits size: {}", limits.to_le_stream().count());
        }
        Err(error) => {
            error!("Error getting duty cycle limits: {error}");
        }
    }

    // Test getting duty cycle state.
    match ezsp.get_duty_cycle_state().await {
        Ok(state) => {
            info!("Duty cycle state: {state:#04X?}");
        }
        Err(error) => {
            error!("Error getting duty cycle state: {error}");
        }
    }

    // Test calculate SMACS.
    match ezsp
        .calculate_smacs(
            true,
            CertificateData::from([0; 48]),
            PublicKeyData::from([0; 22]),
        )
        .await
    {
        Ok(()) => {
            info!("Calculated SMACS");
        }
        Err(error) => {
            error!("Error calculating SMACS: {error}");
        }
    }

    // Test start of scan.
    match ezsp
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

    if args.keep_listening {
        transceiver_thread
            .join()
            .expect("Transceiver thread panicked");
    }
}

/// Test getting values.
async fn get_value_ids<const BUF_SIZE: usize>(ezsp: &mut Ashv2<BUF_SIZE>) {
    for id in [
        Id::ActiveRadioConfig,
        Id::AntennaMode,
        Id::AntennaRxMode,
        Id::ApsFrameCounter,
        Id::Certificate283K1,
        Id::CcaThreshold,
        Id::CoulombCounterUsage,
        Id::DelayedJoinActivation,
        Id::DescriptorCapability,
        Id::EmbernetPassthroughSourceAddress,
        Id::EndpointFlags,
        Id::FreeBuffers,
        Id::MacFilterList,
        Id::MfglibOptions,
        Id::MfgSecurityConfig,
        Id::NwkFrameCounter,
        Id::VersionInfo,
    ] {
        match ezsp.get_value(id).await {
            Ok(data) => {
                info!("{id:?}: {:#04X}", HexSlice::new(&data));
            }
            Err(error) => {
                error!("Error getting {id:?}: {error}");
            }
        }
    }
}
