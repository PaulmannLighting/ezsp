//! Bootloader handlers.

mod bootload_transmit_complete;
mod incoming_bootload_message;

pub use bootload_transmit_complete::Handler as BootloadTransmitComplete;
pub use incoming_bootload_message::Handler as IncomingBootloadMessage;

/// The handler for the bootloader command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// The handler for the bootload transmit complete command.
    BootloadTransmitComplete(BootloadTransmitComplete),
    /// The handler for the incoming bootload message command.
    IncomingBootloadMessage(IncomingBootloadMessage),
}
