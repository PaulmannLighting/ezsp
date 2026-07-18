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
- `apis-saltans`: enables `apis_saltans_hw` integration for `Ncp`/`Builder` and pulls in `apis-saltans` APS/core/hardware/ZDP crates.
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

EZSP and ASHv2 do not add a protocol-level fragmentation layer. On UART, one
EZSP frame is carried in exactly one ASHv2 DATA payload, and one ASHv2 DATA
payload is decoded as exactly one EZSP frame.

The standalone `Defragmenter<T>` reassembles APS-level fragmented unicasts for
any `T` that implements `Messaging`. Its asynchronous `handle` method consumes
each `IncomingMessage`, sends the empty `sendReply` required for fragments, and
returns a `Defragmented` message once the payload is complete. The helper is not
yet wired into `Ncp` or the `apis-saltans` event actor.

Reassembly follows the EZSP fragment window, keys messages by sender and APS
sequence, bounds the payload to 4096 bytes, and expires incomplete messages
after a five-second timeout. The compile-time environment variables
`EZSP_DEFRAGMENTATION_MAX_INCOMING_PACKETS`,
`EZSP_DEFRAGMENTATION_DEFAULT_WINDOW_SIZE`,
`EZSP_DEFRAGMENTATION_RECEIVE_BUFFER_LENGTH`, and
`EZSP_DEFRAGMENTATION_REASSEMBLY_TIMEOUT_MILLIS` override these defaults.

The `ashv2` feature delegates this link layer to the `ashv2` crate and keeps the
EZSP-specific work in this crate:

- `uart::Encoder` serializes EZSP headers and parameters into ASHv2 payloads.
- `uart::Decoder` parses ASHv2 payloads back into typed EZSP frames.
- `uart::Splitter` routes normal responses to the pending request path and UART
  asynchronous callbacks to the callback channel. Its future returns an error if
  one of those destination channels closes before the splitter finishes.
- `uart` re-exports the ASHv2 types and helpers used by the public transport
  API: `FlowControl`, `Handle`, `NativeSerialPort`, `Payload`, `SerialPort`,
  `open`, and `start`.

## Core API

The crate is transport-first:

- `Transport` defines the low-level async connection and request/response primitives.
- EZSP command traits (`Configuration`, `Messaging`, `Networking`, `Security`, ...) are blanket-implemented for any `T: Transport`.
- `Ezsp` is a convenience trait that combines all command traits.
- `Ncp<T>` wraps a transport and adds host-side NCP helpers for scans, APS send
  confirmation, transaction/message sequence counters, and callback correlation.
- Protocol types are exposed through `ember`, `ezsp`, and the typed frame/parameter model.

Implementing `Transport` gives access to the full typed command surface.

## `ashv2` transport

The crate currently ships one concrete transport implementation: `uart::Uart` (`feature = "ashv2"`).

`Uart` provides:

- protocol negotiation through `Transport::connect()` / `Transport::ensure_connection()`
- typed EZSP request/response handling over ASHv2 payload framing
- response/callback demultiplexing
- caller-driven transport futures through `uart::Futures`
- serial constructors:
  - `Uart::open(path, flow_control, protocol_version, &ChannelSizes)` returns
    `(Uart, callbacks, Futures<_>)`
  - `Uart::from_serial_port(serial_port, protocol_version, &ChannelSizes)`
    returns `(Uart, callbacks, Futures<_>)`
  - `Uart::new(handle, ash_rx, callbacks_tx, protocol_version, channel_size)`
    returns `(Uart, splitter_future)` for advanced integration. The splitter
    future resolves to `std::io::Result<()>`.

Additional types:

- `uart::ChannelSizes` to tune queue capacities for `Uart::open` / `from_serial_port`
- `uart::Buffers` for ASHv2 queue sizing in integration helper constructors
- `uart::Futures` for the serial worker, ASHv2 transmitter/receiver, and EZSP
  frame splitter futures that the caller must poll or spawn. The frame splitter
  future resolves to `std::io::Result<()>`.
- `uart::SerialPort`, `uart::FlowControl`, and the other
  re-exported ASHv2 items needed to integrate the UART transport without
  importing `ashv2` paths directly

### Minimal `ashv2` usage

```rust
use ezsp::uart::{ChannelSizes, SerialPort, Uart};
use ezsp::{Transport, Utilities};
use tokio::task::LocalSet;

// Requires feature = "ashv2"
// Requires a Tokio runtime. The returned futures must be driven by the caller.
async fn example() -> Result<(), ezsp::Error> {
    let serial_port = /* your serial port implementing SerialPort */;
    let sizes = ChannelSizes::default();

    let (mut uart, _callbacks, futures) =
        Uart::from_serial_port(serial_port, ezsp::MIN_NON_LEGACY_VERSION, &sizes)?;

    let local = LocalSet::new();
    local.spawn_local(futures.serial_worker);
    local.spawn_local(futures.ash_transmitter);
    local.spawn_local(futures.ash_receiver);
    local.spawn_local(async move {
        futures
            .frame_splitter
            .await
            .expect("EZSP frame splitter failed");
    });

    local
        .run_until(async move {
            uart.ensure_connection().await?;
            let _eui64 = uart.get_eui64().await?;
            Ok(())
        })
        .await
}
```

## NCP Helper

`Ncp<T>` is the high-level host helper for an EZSP Network Co-Processor. It owns
the transport and dereferences to it, so all normal command traits remain
available.

`Ncp` adds behavior that needs more than a single command/response exchange:

- active and energy scans with callback aggregation,
- neighbor table collection,
- unicast and multicast APS sends with `messageSent` confirmation,
- broadcast APS sends with immediate confirmation,
- source endpoint selection for outgoing APS frames from the configured local
  endpoint output clusters,
- message tag and APS sequence counters,
- clean event-handler shutdown through `Ncp::terminate()`.

`Ncp::build(transport, callbacks)` returns `Builder<T>`. The builder stores
policies, configuration values, security keys, APS options, concentrator
settings, network formation settings, and channel buffer sizing for the startup
implementation.

Outgoing APS helper methods take the APS profile ID, cluster ID, destination
endpoint, and message payload. They derive the source endpoint from the first
configured local endpoint that advertises the cluster ID as an output cluster.
If no endpoint matches, the send fails with `Error::NoMatchingSourceEndpoint`.

If `ashv2` is enabled, `Ncp::ashv2(serial_port)` and
`Builder::<uart::Uart>::ashv2(serial_port)` create a builder backed by the
crate's ASHv2 UART transport. The serial port type is constrained by the
re-exported `uart::SerialPort` trait. These constructors return the builder and
the `uart::Futures` set that must be driven alongside the NCP.

## `apis-saltans` Integration (`apis-saltans` Feature)

When `apis-saltans` is enabled, the crate adapts `Ncp` to the
`apis_saltans_hw` driver traits and provides custom `Builder` startup helpers.

- `Ncp<T>: apis_saltans_hw::Driver` when
  `T: Configuration + Security + Messaging + Networking + Utilities + Send + Sync`.
- `Builder::start(endpoints)` configures the EZSP stack, starts callback
  translation, registers each `SimpleDescriptor` as an EZSP endpoint, stores
  the descriptor cluster lists for later source endpoint selection, spawns the
  NCP actor, and returns
  `(apis_saltans_hw::NcpHandle, tokio::sync::mpsc::Receiver<apis_saltans_hw::Event>)`.
- `Ncp::terminate()` stops the event handler and returns the underlying transport.

The integration layer translates EZSP callbacks into `apis_saltans_hw::Event`,
including network-up/down/open/closed events, child join/leave events,
trust-center join/rejoin/leave events, and incoming APS messages. It also
aggregates scan callbacks for `Driver` scan calls and correlates
`messageSent` callbacks with outgoing message tags. Outgoing `Driver` frames
use the frame metadata for the APS profile and cluster; unicast calls use the
requested destination endpoint, while multicast and broadcast calls use the
profile's broadcast endpoint. Unicast sends target one destination endpoint per
call; callers that need fan-out should issue multiple unicast requests.

## Legal

This project is free software and is not affiliated with Silicon Labs. Silicon
Labs documentation is cited only to describe the public protocol implemented by
this crate.

## Contribution guidelines

- Format: `cargo +nightly fmt`
- Lint: `cargo clippy`
