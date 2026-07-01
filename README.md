# ezsp

Rust implementation of the EmberZNet Serial Protocol (EZSP).

## Protocol documentation

- <https://docs.silabs.com/zigbee/latest/sisdk-ezsp-reference-guide/>
- <https://docs.silabs.com/zigbee/6.6/em35x/>

## Features

- `ashv2`: enables the ASHv2 serial transport (`uart::Uart`).
- `zigbee`: enables Zigbee host integration (`zigbee::EzspNetworkManager`) and pulls in Zigbee crates from `apis-saltans`.
- `semver`: enables `semver` support for EZSP version APIs.

## Core API

The crate is transport-first:

- `Transport` defines the low-level async connection and request/response primitives.
- EZSP command traits (`Configuration`, `Messaging`, `Networking`, `Security`, ...) are blanket-implemented for any `T: Transport`.
- `Ezsp` is a convenience trait that combines all command traits.

Implementing `Transport` gives access to the full typed command surface.

## `ashv2` transport

The crate currently ships one concrete transport implementation: `uart::Uart` (`feature = "ashv2"`).

`Uart` provides:

- protocol negotiation through `Transport::connect()` / `Transport::ensure_connection()`
- typed EZSP request/response handling over ASHv2 payload framing
- response/callback demultiplexing
- serial constructors:
  - `Uart::open(path, flow_control, protocol_version, &ChannelSizes)`
  - `Uart::from_serial_port(serial_port, protocol_version, &ChannelSizes)`
  - `Uart::new(proxy, ash_rx, callbacks_tx, protocol_version, channel_size)` (advanced integration)

Additional types:

- `uart::ChannelSizes` to tune queue capacities for `Uart::open` / `from_serial_port`
- `uart::Buffers` for Zigbee builder ASHv2 helper constructors

### Minimal `ashv2` usage

```rust
use ezsp::uart::{ChannelSizes, Uart};
use ezsp::{Transport, Utilities};

// Requires feature = "ashv2"
// Requires a Tokio runtime.
async fn example() -> Result<(), ezsp::Error> {
    let serial_port = /* your serial port implementing ashv2::SerialPort */;
    let sizes = ChannelSizes::default();

    let (mut uart, _ash_tasks, _callbacks) =
        Uart::from_serial_port(serial_port, ezsp::MIN_NON_LEGACY_VERSION, &sizes)?;

    uart.ensure_connection().await?;
    let _eui64 = uart.get_eui64().await?;
    Ok(())
}
```

## Zigbee integration (`zigbee` feature)

When `zigbee` is enabled, the crate exposes `zigbee::EzspNetworkManager<T>`.

- `EzspNetworkManager<T>: zigbee_hw::NcpDriver` when
  `T: Configuration + Security + Messaging + Networking + Utilities + Send + Sync`.
- `EzspNetworkManager::build(transport, callbacks)` returns `Builder<T>`.
- `Builder<T>: zigbee_hw::Start` when `T: Transport + Sync + 'static`.
- `Start::start(...)` configures the NCP, starts callback translation, and returns
  `(zigbee_hw::NcpHandle, tokio::sync::mpsc::Receiver<zigbee_hw::Event>)`.

If both `zigbee` and `ashv2` are enabled, convenience constructors are available:

- `zigbee::EzspNetworkManager::ashv2(serial_port)`

## Legal

This project is free software and is not affiliated with Silicon Labs.

## Contribution guidelines

- Format: `cargo +nightly fmt`
- Lint: `cargo clippy`
