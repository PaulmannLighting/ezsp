use crate::{Error, Transport};
use std::future::Future;

pub trait Binding: Transport {
    fn binding_is_active(&mut self, index: u8) -> impl Future<Output = Result<bool, Error>>;
}
