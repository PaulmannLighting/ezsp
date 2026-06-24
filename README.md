# ezsp

Rust implementation of the EmberZNet Serial Protocol (EZSP).

The project is now mostly feature-complete for core EZSP host-side functionality.

## Protocol Documentation

Reference documentation:

- <https://docs.silabs.com/zigbee/latest/sisdk-ezsp-reference-guide/>
- <https://docs.silabs.com/zigbee/6.6/em35x/>

## Features

- `ashv2`: enables the ASHv2-based serial transport (`uart::Uart`).
- `zigbee`: enables optional Zigbee host integration (`zigbee::EzspNetworkManager`) and pulls in Zigbee crates from `apis-saltans`.
- `semver`: enables `semver` support for version-related APIs.

## Core EZSP API

The core API is trait-based:

- `Transport` is the transport abstraction used by EZSP command traits.
- `Configuration`, `Messaging`, `Networking`, `Security`, etc. are blanket-implemented for any `T: Transport`.
- `Ezsp` is the convenience super-trait combining all command traits.

In other words, implementing `Transport` gives access to the full command surface via blanket impls.

## `ashv2` Transport (Current Primary Implementation)

The only transport implementation currently provided by this crate is `uart::Uart` behind the `ashv2` feature.
This is the primary and intended transport path. A SPI transport is currently not planned.
The transmitter abstraction exposed by this crate is `Transport`, and `Uart` is currently its only implementation.

`Uart` provides:

- protocol version negotiation via `Ezsp::init()`
- automatic connection handling via `Transport::ensure_connection()`
- response/callback demultiplexing
- serial helpers:
  - `Uart::open(path, flow_control, protocol_version, &ChannelSizes)`
  - `Uart::from_serial_port(serial_port, protocol_version, &ChannelSizes)`
  - `Uart::new(proxy, ash_rx, callbacks_tx, protocol_version, channel_size)` for advanced integration

Extended API exposed with `ashv2`:

- `uart::ChannelSizes` for tuning channel capacities used by `Uart::open` / `from_serial_port`.
- `uart::Buffers` for tuning capacities when constructing Zigbee integration builders via ASHv2 helpers.
- `zigbee::EzspNetworkManager::ashv2(...)` and `Builder::<Uart>::ashv2(...)` (when `zigbee` is enabled too).

### Minimal `ashv2` Usage

```rust
use ezsp::uart::{ChannelSizes, Uart};
use ezsp::{Configuration, Ezsp, Utilities};

// Requires feature = "ashv2"
// Requires a Tokio runtime.
async fn example() -> Result<(), ezsp::Error> {
    let serial_port = /* your serial port implementing ashv2::SerialPort */;
    let channel_sizes = ChannelSizes::default();
    let (mut uart, _tasks, _callbacks) =
        Uart::from_serial_port(serial_port, ezsp::MIN_NON_LEGACY_VERSION, &channel_sizes)?;

    uart.init().await?;
    let _eui64 = uart.get_eui64().await?;
    Ok(())
}
```

## Optional Zigbee Integration (`zigbee` Feature)

When `zigbee` is enabled, this crate exposes `zigbee::EzspNetworkManager<T>` for higher-level Zigbee NCP operation.
This integration depends on `apis-saltans` crates (`zigbee`, `zigbee-hw`, `aps`, `zdp`).

### How Types and Traits Work Together

- `EzspNetworkManager<T>` wraps an EZSP transport and event pipeline.
- `EzspNetworkManager<T>: zigbee_hw::NcpDriver` when
  `T: Configuration + Security + Messaging + Networking + Utilities + Send + Sync`.
- `EzspNetworkManager::build(transport, callbacks)` creates `Builder<T>`.
- `Builder<T>: zigbee_hw::Start` when `T: Transport + Sync + 'static`.
- `Start::start(...)` configures the NCP, spawns event translation, and returns `(zigbee_hw::NcpHandle, tokio::sync::mpsc::Receiver<zigbee_hw::Event>)`.
- Incoming EZSP callbacks are translated to `zigbee_hw::Event` via `EventHandler` (`zigbee_hw::EventTranslator`).

### Zigbee Integration Flow

1. Create a transport (`Uart` with `ashv2`, or your own `Transport` implementation).
2. Provide the EZSP callback receiver to `EzspNetworkManager::build(...)`.
3. Configure via builder methods (`with_configuration`, `with_policy`, `with_radio_channel`, `with_link_key`, etc.).
4. Call `start(endpoints)` from `zigbee_hw::Start` to get an `NcpHandle` and event stream.

If both `zigbee` and `ashv2` are enabled, you can use the convenience constructor:

- `EzspNetworkManager::ashv2(serial_port)`

## Legal

This project is free software and is not affiliated with Silicon Labs.

## Contribution Guidelines

- Format: `cargo +nightly fmt`
- Lint: `cargo clippy`
