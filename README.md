# ezsp

Rust implementation of the EmberZNet Serial Protocol (EZSP) host side.

EZSP is the command protocol used by a host application processor to communicate
with the EmberZNet PRO stack running on a Silicon Labs Network Co-Processor
(NCP). The crate models the EZSP command/response surface, frame headers,
parameter payloads, asynchronous callbacks, and the UART transport used by
EZSP-UART NCP firmware.

## Documentation Basis

The crate documentation is based on these Silicon Labs references:

- `UG100: EZSP Reference Guide`, Rev. 5.1, for EmberZNet PRO 7.4.2.
- `UG101: UART-EZSP Gateway Protocol Reference`, Rev. 1.3, for ASHv2 over UART.
- <https://docs.silabs.com/zigbee/latest/sisdk-ezsp-reference-guide/>, which
  currently documents the Simplicity SDK EZSP guide for Zigbee 9.1.0 /
  EmberZNet PRO 8.1 and notes the v8 API/type renaming split from UG100.
- <https://docs.silabs.com/zigbee/6.6/em35x/>, the older EmberZNet API
  reference used by several Ember type descriptions.

The implementation keeps the legacy EZSP naming used throughout UG100 and the
EmberZNet 6.x/7.x API surface where that naming is reflected in the crate.

## Features

- `ashv2`: enables the ASHv2 serial transport (`uart::Uart`).
- `apis-saltans`: enables Zigbee host integration (`apis_saltans::EzspNetworkManager`) and pulls in `apis-saltans` APS/core/hardware/ZDP crates.
- `semver`: enables `semver` support for EZSP version APIs.

## Protocol Model

EZSP messages are exchanged between a host and an NCP over SPI or UART. This
crate currently provides the UART path via ASHv2.

- The host starts normal EZSP use by sending the `version` command after NCP
  reset. A successful version transaction establishes the protocol version that
  both sides will use.
- EZSP fields wider than one byte are serialized little-endian, including EUI64
  values.
- Frames contain a sequence number, frame control, frame ID, and typed
  parameters. EZSP protocol versions before 8 use a three-byte legacy header;
  versions 8 and newer use the extended five-byte header.
- Most commands form a two-message transaction: host command, then NCP response.
  UART NCPs may also send callbacks asynchronously as they occur.
- The response frame control reports NCP status bits such as overflow,
  truncation, callback-pending state, and callback type.

## UART and ASHv2

UG101 describes ASH as the data-link layer below EZSP and above the serial
driver. ASHv2 frames add reliability around EZSP payloads: CRC validation, byte
stuffing, data-field randomization, sliding-window acknowledgements, ACK/NAK
frames, reset handling, and optional not-ready flow control.

The `ashv2` feature delegates this link layer to the `ashv2` crate and keeps the
EZSP-specific work in this crate:

- `uart::Encoder` serializes EZSP headers and parameters into ASHv2 payloads.
- `uart::Decoder` parses ASHv2 payloads back into typed EZSP frames.
- `uart::Splitter` routes normal responses to the pending request path and UART
  asynchronous callbacks to the callback channel.

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

This project is free software and is not affiliated with Silicon Labs. Silicon
Labs documentation is cited only to describe the public protocol implemented by
this crate.

## Contribution guidelines

- Format: `cargo +nightly fmt`
- Lint: `cargo clippy`
