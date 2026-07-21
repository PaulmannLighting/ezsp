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
//! `ashv2::SerialPort`. The builder starts the underlying `ASHv2` worker tasks
//! and combines this module's [`Transmitter`] and [`Receiver`] into the core
//! actor-based [`Transceiver`](crate::Transceiver).

pub use self::receiver::Receiver;
pub use self::transmitter::Transmitter;

mod builder;
mod receiver;
mod transmitter;
