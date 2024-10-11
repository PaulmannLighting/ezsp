#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CallbackType {
    Reserved,
    Async,
    Sync,
}
