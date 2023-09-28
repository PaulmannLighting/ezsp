use std::sync::atomic::AtomicU32;

pub struct Request {
    sequence: AtomicU32,
}
