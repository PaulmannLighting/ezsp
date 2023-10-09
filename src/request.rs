use std::sync::atomic::AtomicU32;

#[derive(Debug)]
pub struct Request {
    sequence: AtomicU32,
}
