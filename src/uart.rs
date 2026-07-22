//! EZSP-UART transport built on `ASHv2`.
//!
//! Silicon Labs describes `ASHv2` as the data-link layer below EZSP for UART
//! gateway systems. `ASHv2` carries EZSP DATA payloads and supplies reset
//! handling, CRC validation, byte stuffing, data-field randomization, ACK/NAK
//! frames, and sliding-window retransmission.
//!
//! This module delegates `ASHv2` link handling to the `ashv2` crate and handles
//! the EZSP-specific pieces: encoding complete command frames, decoding typed
//! response and callback frames, and updating the header decoder after version
//! negotiation.
//!
//! Use [`Builder::ashv2`](crate::Builder::ashv2) with a serial port implementing
//! `ashv2::SerialPort`. It returns the builder together with five transport
//! futures. The caller must spawn the `ASHv2` serial worker, `ASHv2` transmitter,
//! `ASHv2` receiver, EZSP transmitter, and EZSP receiver, in that order, before
//! awaiting [`Builder::start`](crate::Builder::start). The builder connects the
//! internal UART transmit and receive halves to the generic EZSP actors.

pub use self::futures::Futures;
use self::receiver::AshRx;
use self::transmitter::AshTx;

mod builder;
mod futures;
mod receiver;
mod transmitter;
