/// Connection status of transport layer.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Connection {
    /// The transport layer is disconnected.
    #[default]
    Disconnected,

    /// The transport layer is connected.
    Connected,

    /// The transport layer connection failed.
    Failed,
}
