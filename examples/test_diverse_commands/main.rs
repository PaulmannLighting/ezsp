//! Test diverse `EZSP` commands.

use ashv2::{BaudRate, HexSlice, open};
use clap::Parser;
use ezsp::ember::{CertificateData, PublicKeyData};
use ezsp::ezsp::value::Id;
use ezsp::uart::Uart;
use ezsp::{Callback, Cbke, Configuration, Networking, ProxyTable, Security, SinkTable, Utilities};
use le_stream::ToLeStream;
use log::{debug, error, info};
use serialport::{FlowControl, SerialPort};
use silizium::zigbee::security::man::{Context, DerivedKeyType, Flags, KeyType};
use tokio::sync::mpsc::channel;

use duty_cycle::get_duty_cycle_info;
use get_keys::get_keys;

mod duty_cycle;
mod get_keys;

const TEST_TEXT: &str = "Rust rules! 🦀";

#[derive(Debug, Parser)]
struct Args {
    #[arg(index = 1, help = "Path to the serial port")]
    tty: String,
    #[arg(short, long, help = "EZSP version to negotiate")]
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

#[allow(clippy::too_many_lines)]
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

    let mut ezsp = Uart::new(serial_port, callbacks_tx, args.version, 8);

    // Test echo reply. Should be same as sent text.
    match ezsp.echo(TEST_TEXT.bytes().collect()).await {
        Ok(bytes) => match String::from_utf8(bytes.to_vec()) {
            Ok(text) => {
                info!("Got echo: {text}");
                assert_eq!(text, TEST_TEXT);
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
    let eui64 = match ezsp.get_eui64().await {
        Ok(eui64) => {
            info!("EUI64: {eui64}");
            eui64
        }
        Err(error) => {
            error!("Error getting EUI64: {error}");
            return;
        }
    };

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

    // Test duty-cycle-related commands.
    get_duty_cycle_info(&mut ezsp).await;

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

    // Test key export.
    let context = Context::new(
        KeyType::Network,
        0,
        DerivedKeyType::LoadKey,
        eui64,
        0,
        Flags::empty(),
        0,
    );
    match ezsp.export_key(context).await {
        Ok(key) => {
            info!("Exported key: {key:#04X?}");
        }
        Err(error) => {
            error!("Error exporting key: {error}");
        }
    }

    // Test getting keys.
    get_keys(&mut ezsp).await;

    // Test nop.
    if let Err(error) = ezsp.nop().await {
        error!("Nop error: {error}");
    } else {
        info!("Nop succeeded");
    }

    // Test getting callbacks
    match ezsp.callback().await {
        Ok(callback) => {
            info!("Callback: {callback:?}");
        }
        Err(error) => {
            error!("Error getting callback: {error}");
        }
    }

    // Test read counters
    match ezsp.read_counters().await {
        Ok(counters) => {
            info!("Counters: {counters:#06X?}");
        }
        Err(error) => {
            error!("Error getting counters: {error}");
        }
    }

    // Test get network parameters
    match ezsp.get_network_parameters().await {
        Ok((typ, parameters)) => {
            info!("Network type: {typ}");
            info!("Network parameters: {parameters:?}");
        }
        Err(error) => {
            error!("Error getting network parameters: {error}");
        }
    }
}

/// Test getting values.
async fn get_value_ids<T>(ezsp: &mut Uart<T>)
where
    T: SerialPort + 'static,
{
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
