//! EZSP-UART transport built on `ASHv2`.
//!
//! Silicon Labs describes `ASHv2` as the data-link layer below EZSP for UART
//! gateway systems. `ASHv2` carries EZSP DATA payloads and supplies reset
//! handling, CRC validation, byte stuffing, data-field randomization, ACK/NAK
//! frames, and sliding-window retransmission.
//!
//! This module delegates `ASHv2` link handling to the `ashv2` crate and handles
//! the EZSP-specific pieces: protocol-version negotiation, EZSP header
//! selection, typed request/response exchange, and asynchronous callback
//! demultiplexing.
//!
//! The `ASHv2` types used by the public constructors are re-exported from this
//! module. Users can import [`FlowControl`], [`Handle`], [`NativeSerialPort`],
//! [`Payload`], [`SerialPort`], [`open`], and [`start`] from
//! `ezsp::uart` without naming the underlying transport crate directly.

pub use self::receiver::Receiver;
pub use self::transmitter::Transmitter;

mod builder;
mod receiver;
mod transmitter;
