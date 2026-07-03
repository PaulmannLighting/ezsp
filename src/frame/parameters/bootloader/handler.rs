//! Bootloader handlers.

pub use self::bootload_transmit_complete::Handler as BootloadTransmitComplete;
pub use self::incoming_bootload_message::Handler as IncomingBootloadMessage;

mod bootload_transmit_complete;
mod incoming_bootload_message;
crate::frame::parameters::parameter_enum!(
    Handler,
    BootloadTransmitComplete,
    IncomingBootloadMessage
);
