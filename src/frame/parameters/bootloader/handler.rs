//! Bootloader handlers.

pub use self::bootload_transmit_complete::Handler as BootloadTransmitComplete;
pub use self::incoming_bootload_message::Handler as IncomingBootloadMessage;

mod bootload_transmit_complete;
mod incoming_bootload_message;

/// The handler for the bootloader command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// The handler for the bootload transmit complete command.
    BootloadTransmitComplete(BootloadTransmitComplete),
    /// The handler for the incoming bootload message command.
    IncomingBootloadMessage(IncomingBootloadMessage),
}
