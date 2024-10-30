/// Connection status of the UART.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Connection {
    #[default]
    Disconnected,
    Connected,
    Failed,
}
