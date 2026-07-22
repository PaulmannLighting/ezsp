//! Actor-based EZSP transport orchestration.
//!
//! [`Transmitter`] and [`Receiver`] are independent futures connected by
//! bounded channels. Transport constructors return those futures to the caller,
//! which must spawn them before negotiating through [`Connectable`]. The
//! receiver routes response frames back to the transmitter and asynchronous
//! callbacks to a separate stream. After version negotiation, the cloneable
//! [`Connection`] handle provides typed command/response transactions.

pub use self::connectable::Connectable;
pub use self::connection::Connection;
pub use self::message::Message;
pub use self::receiver::{Receive, Receiver};
pub use self::translatable_event::TranslatableEvent;
pub use self::transmitter::{Transmit, Transmitter};

mod connectable;
mod connection;
mod message;
mod receiver;
mod translatable_event;
mod transmitter;
