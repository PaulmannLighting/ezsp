# ezsp

Actor-based host support for the EmberZNet Serial Protocol (EZSP).

EZSP is the command protocol used by a host application processor to control
the EmberZNet PRO stack running on a Silicon Labs Network Co-Processor (NCP).
This crate models typed command, response, and callback payloads; legacy and
extended frame headers; transport-independent actors; high-level Zigbee
workflows; and an optional ASHv2 UART transport.

## Documentation basis

The protocol model follows these Silicon Labs references:

- `UG100: EZSP Reference Guide`, Rev. 5.1, for EmberZNet PRO 7.4.2.
- `UG101: UART-EZSP Gateway Protocol Reference`, Rev. 1.3, for ASHv2 over UART.
- <https://docs.silabs.com/zigbee/latest/sisdk-ezsp-reference-guide/>, the
  current Simplicity SDK EZSP reference.
- <https://docs.silabs.com/zigbee/6.6/em35x/>, the older EmberZNet API
  reference used by several Ember type descriptions.

The implementation retains the legacy EZSP and Ember names where they are part
of the crate API.

## Features

- `ashv2` provides `Builder::ashv2` for EZSP over an ASHv2 serial link and
  re-exports the transport crate as `ezsp::ashv2`.
- `apis-saltans` implements `apis_saltans_hw::Driver` for suitable `Ncp<T>`
  values and supplies callback/event and data-model conversions.
- `semver` enables `semver` support in EZSP version APIs.

## Actor model

The transport API separates outbound and inbound I/O:

- `Transmit` sends a complete `Frame<Commands>`.
- `Receive` yields decoded `Frame<Parameters>` values and accepts the negotiated
  EZSP version used by version-sensitive decoders.
- `Transceiver<T, R>` joins those halves and spawns a transmitter actor plus a
  receiver task.
- `Disconnected` represents the running tasks before protocol negotiation.
- `Disconnected::connect` sends the initial `version` command and returns a
  cloneable `Connected` handle together with the asynchronous callback stream.
- `Connected` implements `Communicate`; all EZSP command-group traits are
  blanket-implemented for communicators.

Every `Connected::communicate` call sends an actor message and waits on its own
one-shot response. The transmitter actor assigns an EZSP sequence number,
serializes outbound access, and correlates inbound responses by that number.
Cloned handles can therefore be used by independent tasks without placing the
transport behind a mutex. Asynchronous callbacks bypass response correlation
and are delivered through a separate bounded channel.

```mermaid
flowchart LR
    callers[Connected handle clones] --> commands[Actor inbox]
    commands --> transmitter[Transmitter actor]
    transmitter --> tx[Transmit implementation]
    tx --> ncp[NCP]
    ncp --> rx[Receive implementation]
    rx --> receiver[Receiver task]
    receiver --> commands
    receiver --> callbacks[Callback stream]
```

Custom transports implement `Transmit` and `Receive` independently:

```rust
use std::num::NonZero;

use ezsp::{Transceiver, Utilities};

async fn use_transport<T, R>(transmit: T, receive: R) -> Result<(), ezsp::Error>
where
    T: ezsp::Transmit + Send + 'static,
    R: ezsp::Receive + Send + 'static,
{
    let disconnected = Transceiver::new(transmit, receive).spawn(128, 64);
    let desired_version = NonZero::new(ezsp::MIN_NON_LEGACY_VERSION)
        .expect("the minimum non-legacy version is non-zero");
    let (mut connected, mut callbacks) = disconnected.connect(desired_version).await?;

    let _callback_task = tokio::spawn(async move {
        while let Some(callback) = callbacks.recv().await {
            // Dispatch callbacks required by the application.
            drop(callback);
        }
    });

    let _eui64 = connected.get_eui64().await?;
    Ok(())
}
```

## Typed protocol API

EZSP command methods are grouped into traits such as `Configuration`,
`Messaging`, `Networking`, `Security`, and `Utilities`. Each method creates a
typed command parameter, calls `Communicate::communicate`, and converts the
correlated response into its public return type. `Ezsp` is a convenience trait
combining the complete command surface.

The lower-level frame model remains public for transport implementations and
protocol tooling:

- `Frame`, `Header`, `Legacy`, and `Extended` model the EZSP envelope.
- `Commands` is the internal aggregate used by `Transmit` implementations.
- `Parameters`, `Response`, and `Callback` classify decoded inbound payloads.
- Protocol data types are exposed through `ember`, `ezsp`, and the typed
  parameter modules.

EZSP fields wider than one byte are encoded little-endian. Protocol versions
before 8 use the three-byte legacy header; versions 8 and newer use the
five-byte extended header. The receiver updates its decoder after a successful
`version` response.

## High-level NCP startup

`Builder<T, R>` owns a `Transceiver` and the complete startup configuration. Its
`start` method:

1. validates that at least one application endpoint was supplied;
2. spawns the transport actors and negotiates the requested EZSP version;
3. applies concentrator, configuration, and policy settings;
4. resumes the persisted network or forms an explicitly configured network;
5. waits for `NetworkUp`, applies runtime radio power, and sends a many-to-one
   route request;
6. registers the supplied endpoints;
7. starts callback translation, scan aggregation, APS defragmentation, and
   message-confirmation correlation;
8. returns `Ncp<Connected>`.

The event channel passed to `Builder::start` determines the application event
type. That type must implement `TranslatableEvent`, which is automatically
implemented for types that can be constructed from both `Callback` and
`DefragmentedMessage`.

Builder methods configure callback and actor channel capacities, the desired
protocol version, EZSP policies and configuration values, concentrator
parameters, radio transmit power, and default APS options.

`Startup::Resume` restores state persisted by the NCP through `networkInit` and
is the normal choice for restarts:

```rust
use ezsp::Startup;
use ezsp::ezsp::network::InitBitmask;

let startup = Startup::Resume(InitBitmask::NO_OPTIONS);
```

`Startup::Initialize` intentionally replaces the current network. It attempts
to leave the current network, installs the initial security state, and forms a
network from `InitializationParameters`. `NetworkCredentials` groups the
extended PAN ID, PAN ID, trust-center EUI-64, and network key; initialization
parameters add the preconfigured trust-center link key, channel, join method,
and initial security bitmask.

```rust
use ezsp::{InitializationParameters, NetworkCredentials, Startup};

let credentials = NetworkCredentials::new(
    extended_pan_id,
    pan_id,
    trust_center_eui64,
    network_key,
);
let parameters = InitializationParameters::new(
    credentials,
    link_key,
    radio_channel,
    join_method,
    security_bitmask,
);
let startup = Startup::Initialize(parameters);
```

`NetworkCredentials` contains secret key material. Do not log its `Debug`
output, and protect persisted or copied credentials appropriately. Random
credentials can be sampled with `rand`, but the distribution accepts any RNG;
production callers are responsible for selecting a cryptographically secure
one.

## High-level NCP operations

`Ncp<T>` owns the connected communicator and endpoint metadata. It adds
workflows that span commands and asynchronous callbacks:

- active-network and energy scans, completed by `scanComplete`;
- unicast, multicast, and broadcast APS sends;
- outgoing message-tag correlation with `messageSent` callbacks;
- incoming APS fragment reassembly;
- source-endpoint selection from registered output clusters; and
- event-handler shutdown through `Ncp::terminate`.

Outgoing APS sends select the lowest-numbered registered local endpoint whose
output clusters contain the requested cluster ID. ZDP uses endpoint zero. A
missing match returns `Error::NoMatchingSourceEndpoint` before a send command is
issued.

Awaiting `Ncp::unicast`, `Ncp::multicast`, or `Ncp::broadcast` performs the EZSP
send transaction and returns a deferred `StackResponse` (multicast also returns
the assigned APS sequence). Await `StackResponse` separately to validate the
matching asynchronous `messageSent` callback. Dropping it discards only the
notification and does not cancel a message already accepted by the NCP.

Oversized unicasts are split into APS fragments. Multicast and broadcast
payloads must fit the maximum payload reported by the NCP.

## APS defragmentation

`Defragmenter<T>` reassembles fragmented incoming APS unicasts for any
`T: Messaging`. `handle` acknowledges each fragment with the required empty
`sendReply` and returns a `DefragmentedMessage` after the complete payload is
available. The high-level event handler owns a defragmenter using its clone of
the `Connected` actor handle and emits incoming-message events only for complete
payloads.

Reassembly keys messages by sender and APS sequence, enforces the fragment
window and receive-buffer limits, and expires incomplete messages. Compile-time
environment variables can override the defaults:

- `EZSP_DEFRAGMENTATION_MAX_INCOMING_PACKETS`
- `EZSP_DEFRAGMENTATION_DEFAULT_WINDOW_SIZE`
- `EZSP_DEFRAGMENTATION_RECEIVE_BUFFER_LENGTH`
- `EZSP_DEFRAGMENTATION_REASSEMBLY_TIMEOUT_MILLIS`

## ASHv2 UART transport

With `ashv2` enabled, the internal UART transmitter encodes complete EZSP frames
into ASHv2 DATA payloads and its receiver decodes DATA payloads into typed
frames. `Builder::ashv2(serial_port)` constructs both halves, starts the ASHv2
serial worker/transmitter/receiver tasks, and returns a normal actor builder.
`Builder::ashv2_with_buffers` additionally sets the ASHv2 DATA channel capacity.

```rust
// Requires the `ashv2` feature and a running Tokio runtime.
let builder = ezsp::Builder::ashv2(serial_port)
    .with_callbacks_capacity(128)
    .with_messages_capacity(64);

let ncp = builder
    .start(startup, endpoints, event_sender)
    .await?;
```

The serial port must implement `ezsp::ashv2::SerialPort`. ASHv2 supplies
reliability, CRC validation, byte stuffing, randomization, acknowledgements,
reset handling, and retransmission. Neither EZSP nor ASHv2 fragments protocol
frames: one EZSP frame is encoded into one ASHv2 DATA payload.

## `apis-saltans` integration

The `apis-saltans` feature adds implementations and conversions around the
normal actor-backed `Ncp`; it does not add a wrapper type or another transport.

`Ncp<T>` implements `apis_saltans_hw::Driver` when `T` implements
`Configuration + Messaging + Networking + Utilities + Send + Sync`. The
mapping provides:

- stored endpoint descriptors through `Driver::get_endpoints`;
- NCP identity and EZSP address-table lookup operations;
- active-network and energy scans through the existing callback aggregator;
- permit joining, with the requested duration truncated to whole seconds and
  clamped to 255 seconds;
- high-RAM many-to-one route requests; and
- unicast, broadcast, and multicast datagram transmission through the
  high-level NCP send helpers.

`Builder::start` continues to return `Ncp<Connected>`. To run the separate
`apis-saltans` hardware actor, call `Driver::run` and spawn its returned future:

```rust
use apis_saltans_hw::Driver;

let ncp = builder.start(startup, endpoints, event_sender).await?;
let (hardware, driver) = ncp.run(64);
tokio::spawn(driver);

// `hardware` is an apis_saltans_hw::NcpHandle.
```

The feature provides bidirectional endpoint conversion. Convert the ZDP simple
descriptors used by `apis-saltans` before passing them to the EZSP builder:

```rust
let endpoints: Box<[ezsp::Endpoint]> = simple_descriptors
    .into_iter()
    .map(Into::into)
    .collect();
```

`Driver::get_endpoints` performs the reverse conversion. Endpoints containing
an unsupported `apis-saltans` profile or a reserved endpoint number are logged
and omitted; descriptors originally converted from `SimpleDescriptor` round
trip without that loss.

Outgoing datagrams take their APS profile and cluster from
`apis_saltans_hw::Datagram` metadata. A device destination preserves its target
endpoint. A broadcast uses its target endpoint with radius zero. A group uses
the profile's broadcast endpoint with zero multicast hops and nonmember radius.
The local source endpoint is still selected from the registered EZSP output
clusters.

`Driver::transmit` returns `HwResponse` after the EZSP send transaction has been
accepted. The response contains the deferred `StackResponse`; awaiting it
reports the later `messageSent` callback status.

### Event and message conversion

The feature converts these EZSP callbacks into `apis_saltans_hw::Event` values:

- `stackStatus` for network up, down, opened, and closed;
- `childJoin` for child joins and leaves; and
- `trustCenterJoin` for unsecured joins, secured/unsecured rejoins, and leaves.

Complete incoming APS messages convert separately into
`apis_saltans_hw::aps::Data<bytes::Bytes>` and NWK envelopes. The conversion
preserves APS destination, profile, cluster, endpoints, sequence, and payload,
plus the sender short ID, link quality, RSSI, binding index, and source-route
overhead. The source IEEE address remains unknown.

The feature does not currently implement
`TryFrom<DefragmentedMessage>` directly for `apis_saltans_hw::Event`. Therefore
that event enum alone does not implement `TranslatableEvent` and cannot be used
as `Builder::start`'s event type without an application wrapper that supplies
both required conversions.

EZSP errors cross the driver boundary as
`apis_saltans_hw::Error::Implementation`, retaining the original error in an
`Arc`.

## Legal

This project is free software and is not affiliated with Silicon Labs. Silicon
Labs documentation is cited only to describe the public protocol implemented by
this crate.

## Contributing

- Format with `cargo +nightly fmt` and verify with
  `cargo +nightly fmt --check`.
- Lint with `cargo clippy --all-features`.
- Verify documentation with `cargo +nightly doc --all-features --no-deps`.
