pub mod rx;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    Rx(rx::Handler),
}

impl From<rx::Handler> for Handler {
    fn from(handler: rx::Handler) -> Self {
        Self::Rx(handler)
    }
}
