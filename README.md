# ezsp

Rust implementation of the EmberZNet Serial Protocol (EZSP).

## Protocol documentation

- <https://docs.silabs.com/zigbee/latest/sisdk-ezsp-reference-guide/>
- <https://docs.silabs.com/zigbee/6.6/em35x/>

## Features

- `ashv2`: enables the ASHv2 serial transport (`uart::Uart`).
- `apis-saltans`: enables Zigbee host integration (`apis_saltans::EzspNetworkManager`) and pulls in `apis-saltans` APS/core/hardware/ZDP crates.
- `semver`: enables `semver` support for EZSP version APIs.

## Core API

The crate is transport-first:

- `Transport` defines the low-level async connection and request/response primitives.
- EZSP command traits (`Configuration`, `Messaging`, `Networking`, `Security`, ...) are blanket-implemented for any `T: Transport`.
- `Ezsp` is a convenience trait that combines all command traits.
- Protocol types are exposed through `ember`, `ezsp`, and the typed frame/parameter model.

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
- `Uart::abort()` for aborting the background splitter task

Additional types:

- `uart::ChannelSizes` to tune queue capacities for `Uart::open` / `from_serial_port`
- `uart::Buffers` for ASHv2 queue sizing in integration helper constructors

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

## `apis-saltans` integration (`apis-saltans` feature)

When `apis-saltans` is enabled, the crate exposes `apis_saltans::EzspNetworkManager<T>`.

- `EzspNetworkManager<T>: apis_saltans_hw::NcpDriver` when
  `T: Configuration + Security + Messaging + Networking + Utilities + Send + Sync`.
- `EzspNetworkManager::build(transport, callbacks)` returns `Builder<T>`.
- `Builder<T>: apis_saltans_hw::Start` when `T: Transport + Sync + 'static`.
- `Start::start(...)` configures the NCP, starts callback translation, and returns
  `(apis_saltans_hw::NcpHandle, tokio::sync::mpsc::Receiver<apis_saltans_hw::Event>)`.
- `EzspNetworkManager::terminate()` stops the event handler and returns the underlying transport.

The integration layer translates EZSP callbacks into `apis_saltans_hw::Event`, including network-up/down/open/closed events, child join/leave events, trust-center join/rejoin/leave events, and incoming APS messages. It separately aggregates scan callbacks for `NcpDriver` scan calls and correlates `MessageSent` callbacks with outgoing message tags.

If both `apis-saltans` and `ashv2` are enabled, convenience constructors are available:

- `apis_saltans::EzspNetworkManager::ashv2(serial_port)`

## Legal

This project is free software and is not affiliated with Silicon Labs.

## Contribution guidelines

- Format: `cargo +nightly fmt`
- Lint: `cargo clippy`
