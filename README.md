# ezsp

The EmberZNet Serial Protocol

## Work in Progress

This is a work in progress.

The goal is to provide a Rust implementation of the `EZSP` library as documented below.

## Documentation

The official documentation can be found at

- <https://www.silabs.com/documents/public/user-guides/ug100-ezsp-reference-guide.pdf>
- <https://docs.silabs.com/zigbee/6.6/em35x/>

## Usage

### Raw EZSP frame communication

If you just want to send and receive raw EZSP frames, you can use the EZSP-related traits as exported by this library on
any type which implements `Trnsport`.

This should only be done to investigate and get familiar with the protocol, not for production use.

### Zigbee Host Communication

If you want to use this library to use an EZSP-compatible NCP, you can use the `NetworkManager` struct.

It is recommended to construct it using the `NetworkManager::build()` method, which returns an appropriate builder.

The resulting `NetworkManager` implements the `Actor` trait from the [
`zigbee-nwk`](https://github.com/PaulmannLighting/apis-saltans/tree/main/nwk) crate, which is part of the
[`apis-saltans`](https://github.com/PaulmannLighting/apis-saltans) project.

All of these features require the `zigbee` feature to be enabled.

## Legal

This project is free software and is not affiliated with Siliconlabs.

## Contribution guidelines

* Format the code with `cargo +nightly fmt`
* Check code with `cargo clippy`