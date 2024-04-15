use std::future::Future;

use crate::{Error, Transport};

pub trait Security: Transport {
    fn clear_key_table(&self) -> impl Future<Output = Result<(), Error>>;
}
